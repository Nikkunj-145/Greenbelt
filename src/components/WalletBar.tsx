import { Wallet, LogOut, Copy, RefreshCw } from 'lucide-react';
import { shortAddr } from '../lib/format';

interface Props {
  address: string | null;
  onConnect: () => void;
  onDisconnect: () => void;
  onSwitch: () => void;
  loading: boolean;
}

export function WalletBar({ address, onConnect, onDisconnect, onSwitch, loading }: Props) {
  if (!address) {
    return (
      <button onClick={onConnect} disabled={loading} className="btn btn-primary">
        <Wallet className="w-4 h-4" />
        {loading ? 'Connecting…' : 'Connect Wallet'}
      </button>
    );
  }
  return (
    <div className="flex items-center gap-2">
      <div className="flex items-center gap-2 px-3 py-2 rounded-xl border border-white/10 bg-white/5">
        <span className="w-2 h-2 rounded-full bg-mint" />
        <span className="font-mono text-xs sm:text-sm">{shortAddr(address)}</span>
        <button
          onClick={() => navigator.clipboard.writeText(address)}
          className="p-1 rounded hover:bg-white/10"
          title="Copy"
        >
          <Copy className="w-3.5 h-3.5 text-white/60" />
        </button>
      </div>
      <button
        onClick={onSwitch}
        disabled={loading}
        className="btn btn-ghost text-sm"
        title="Switch account"
      >
        <RefreshCw className="w-4 h-4" />
      </button>
      <button onClick={onDisconnect} className="btn btn-ghost text-sm" title="Disconnect">
        <LogOut className="w-4 h-4" />
      </button>
    </div>
  );
}
