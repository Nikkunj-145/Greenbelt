import { Coins, Lock, Sparkles, Globe2 } from 'lucide-react';
import { fmtTok } from '../lib/format';

interface Props {
  balance: bigint;
  staked: bigint;
  points: bigint;
  total: bigint;
}

export function Stats({ balance, staked, points, total }: Props) {
  return (
    <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
      <div className="stat">
        <div className="stat-label flex items-center gap-1.5">
          <Coins className="w-3.5 h-3.5" /> Wallet
        </div>
        <div className="stat-value text-mint">{fmtTok(balance, false)}</div>
        <div className="text-xs text-white/40 mt-1">STK in your wallet</div>
      </div>
      <div className="stat">
        <div className="stat-label flex items-center gap-1.5">
          <Lock className="w-3.5 h-3.5" /> Staked
        </div>
        <div className="stat-value">{fmtTok(staked, false)}</div>
        <div className="text-xs text-white/40 mt-1">Locked in pool</div>
      </div>
      <div className="stat">
        <div className="stat-label flex items-center gap-1.5">
          <Sparkles className="w-3.5 h-3.5" /> Your Points
        </div>
        <div className="stat-value text-teal">{Number(points).toLocaleString()}</div>
        <div className="text-xs text-white/40 mt-1">Live + claimed</div>
      </div>
      <div className="stat">
        <div className="stat-label flex items-center gap-1.5">
          <Globe2 className="w-3.5 h-3.5" /> Pool TVL
        </div>
        <div className="stat-value">{fmtTok(total, false)}</div>
        <div className="text-xs text-white/40 mt-1">Total staked</div>
      </div>
    </div>
  );
}
