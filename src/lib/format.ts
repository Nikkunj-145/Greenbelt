import { TOKEN_DECIMALS, TOKEN_SYMBOL } from './config';

export function shortAddr(addr: string) {
  if (!addr) return '';
  return `${addr.slice(0, 4)}…${addr.slice(-4)}`;
}
export function explorerTx(h: string) {
  return `https://stellar.expert/explorer/testnet/tx/${h}`;
}
export function explorerContract(id: string) {
  return `https://stellar.expert/explorer/testnet/contract/${id}`;
}
/** Convert raw stroop-like int to display number */
export function fromRaw(n: bigint | number | string): number {
  const v = typeof n === 'bigint' ? Number(n) : Number(n);
  return v / 10 ** TOKEN_DECIMALS;
}
/** Display string for a raw amount (formatted with up to 4 decimals + symbol) */
export function fmtTok(raw: bigint | number | string, withSym = true) {
  const v = fromRaw(raw);
  const s = v.toLocaleString(undefined, { maximumFractionDigits: 4 });
  return withSym ? `${s} ${TOKEN_SYMBOL}` : s;
}
/** Convert display string (e.g. "1.5") to raw bigint */
export function toRaw(display: string): bigint {
  const [whole, frac = ''] = display.split('.');
  const fracPadded = (frac + '0'.repeat(TOKEN_DECIMALS)).slice(0, TOKEN_DECIMALS);
  return BigInt(whole || '0') * BigInt(10 ** TOKEN_DECIMALS) + BigInt(fracPadded || '0');
}
