import { useState } from 'react';
import { ArrowDownToLine, ArrowUpFromLine, Sparkles, Loader2, ExternalLink, AlertCircle, CheckCircle2 } from 'lucide-react';
import { fmtTok, fromRaw, toRaw, explorerTx } from '../lib/format';
import { TOKEN_SYMBOL } from '../lib/config';

export type TxStatus =
  | { kind: 'idle' }
  | { kind: 'pending'; step: string }
  | { kind: 'success'; hash: string; label: string }
  | { kind: 'error'; message: string };

interface Props {
  connected: boolean;
  balance: bigint;
  staked: bigint;
  status: TxStatus;
  onStake: (amount: bigint) => Promise<void>;
  onUnstake: (amount: bigint) => Promise<void>;
  onClaim: () => Promise<void>;
}

export function StakePanel({ connected, balance, staked, status, onStake, onUnstake, onClaim }: Props) {
  const [tab, setTab] = useState<'stake' | 'unstake'>('stake');
  const [amount, setAmount] = useState('');

  const max = tab === 'stake' ? balance : staked;
  const maxDisplay = fromRaw(max);

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const raw = toRaw(amount || '0');
      if (raw <= 0n) return;
      if (raw > max) return;
      if (tab === 'stake') await onStake(raw);
      else await onUnstake(raw);
      if (status.kind !== 'error') setAmount('');
    } catch {}
  };

  const pending = status.kind === 'pending';
  const setMax = () => setAmount(String(maxDisplay));

  return (
    <div className="glass rounded-2xl p-5 sm:p-6">
      <div className="grid grid-cols-2 gap-1 p-1 rounded-xl bg-white/5 mb-5">
        {(['stake', 'unstake'] as const).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`py-2 rounded-lg text-sm font-semibold capitalize transition ${
              tab === t ? 'bg-mint text-ink' : 'hover:bg-white/5'
            }`}
          >
            {t === 'stake' ? <ArrowDownToLine className="w-4 h-4 inline mr-1.5" /> : <ArrowUpFromLine className="w-4 h-4 inline mr-1.5" />}
            {t}
          </button>
        ))}
      </div>

      <form onSubmit={submit} className="space-y-3">
        <div className="flex items-center justify-between text-xs text-white/50">
          <span>Amount ({TOKEN_SYMBOL})</span>
          <button
            type="button"
            onClick={setMax}
            disabled={!connected}
            className="hover:text-mint disabled:opacity-50"
          >
            Max: {fmtTok(max, false)}
          </button>
        </div>
        <div className="relative">
          <input
            value={amount}
            onChange={(e) => setAmount(e.target.value.replace(/[^0-9.]/g, ''))}
            placeholder="0.0"
            inputMode="decimal"
            className="input text-2xl font-bold pr-20"
            disabled={!connected || pending}
          />
          <span className="absolute right-4 top-1/2 -translate-y-1/2 text-sm text-white/40 font-mono">{TOKEN_SYMBOL}</span>
        </div>

        <button
          type="submit"
          disabled={!connected || pending || !amount || toRaw(amount) <= 0n || toRaw(amount) > max}
          className="btn btn-primary w-full"
        >
          {pending ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" /> {status.step}
            </>
          ) : !connected ? (
            'Connect wallet'
          ) : tab === 'stake' ? (
            'Stake'
          ) : (
            'Unstake'
          )}
        </button>

        <button
          type="button"
          onClick={onClaim}
          disabled={!connected || pending || staked === 0n}
          className="btn btn-ghost w-full"
        >
          <Sparkles className="w-4 h-4" />
          Claim points
        </button>
      </form>

      {status.kind === 'success' && (
        <div className="mt-4 rounded-xl border border-mint/30 bg-mint/5 p-3 text-sm">
          <div className="flex items-center gap-2 text-mint font-semibold mb-1">
            <CheckCircle2 className="w-4 h-4" /> {status.label}
          </div>
          <a
            href={explorerTx(status.hash)}
            target="_blank"
            rel="noreferrer"
            className="text-xs text-mint/80 hover:underline inline-flex items-center gap-1"
          >
            View tx <ExternalLink className="w-3 h-3" />
          </a>
        </div>
      )}
      {status.kind === 'error' && (
        <div className="mt-4 rounded-xl border border-red-500/30 bg-red-500/5 p-3 text-sm">
          <div className="flex items-center gap-2 text-red-400 font-semibold mb-1">
            <AlertCircle className="w-4 h-4" /> Failed
          </div>
          <div className="text-white/70 text-xs break-words">{status.message}</div>
        </div>
      )}
    </div>
  );
}
