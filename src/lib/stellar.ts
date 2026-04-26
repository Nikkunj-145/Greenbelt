import {
  rpc,
  Contract,
  TransactionBuilder,
  BASE_FEE,
  Address,
  Account,
  nativeToScVal,
  scValToNative,
  xdr,
} from '@stellar/stellar-sdk';
import { NETWORK_PASSPHRASE, RPC_URL, STAKING_ID, TOKEN_ID } from './config';
import { signXdr } from './wallet';

export const server = new rpc.Server(RPC_URL, { allowHttp: RPC_URL.startsWith('http://') });

const DUMMY = 'GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAOOO';

async function simulate(contractId: string, method: string, args: xdr.ScVal[] = []) {
  const tx = new TransactionBuilder(new Account(DUMMY, '0'), {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(new Contract(contractId).call(method, ...args))
    .setTimeout(60)
    .build();
  const sim = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(sim)) throw new Error(sim.error);
  if (!('result' in sim) || !sim.result) throw new Error('No simulation result');
  return scValToNative(sim.result.retval);
}

async function sendTx(
  caller: string,
  contractId: string,
  method: string,
  args: xdr.ScVal[],
): Promise<{ hash: string; result: any }> {
  const account = await server.getAccount(caller);
  const built = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(new Contract(contractId).call(method, ...args))
    .setTimeout(60)
    .build();
  const prepared = await server.prepareTransaction(built);
  const signed = await signXdr(prepared.toXDR(), caller);
  const tx = TransactionBuilder.fromXDR(signed, NETWORK_PASSPHRASE);
  const send = await server.sendTransaction(tx);
  if (send.status === 'ERROR') throw new Error(`Send failed: ${JSON.stringify(send.errorResult ?? send)}`);
  for (let i = 0; i < 30; i++) {
    await new Promise((r) => setTimeout(r, 1500));
    const status = await server.getTransaction(send.hash);
    if (status.status === 'SUCCESS') {
      let result: any = null;
      try {
        const ret = (status as any).returnValue;
        if (ret) result = scValToNative(ret);
      } catch {}
      return { hash: send.hash, result };
    }
    if (status.status === 'FAILED') throw new Error('Transaction failed on-chain');
  }
  throw new Error('Transaction timed out');
}

// ─── Token reads ───
export async function tokenBalance(addr: string): Promise<bigint> {
  if (!TOKEN_ID) throw new Error('VITE_TOKEN_ID not set');
  const v = await simulate(TOKEN_ID, 'balance', [new Address(addr).toScVal()]);
  return BigInt(v);
}
export async function tokenSymbol(): Promise<string> {
  if (!TOKEN_ID) return '';
  return String(await simulate(TOKEN_ID, 'symbol'));
}

// ─── Staking reads ───
export async function staked(addr: string): Promise<bigint> {
  if (!STAKING_ID) throw new Error('VITE_STAKING_ID not set');
  const v = await simulate(STAKING_ID, 'staked', [new Address(addr).toScVal()]);
  return BigInt(v);
}
export async function pendingPoints(addr: string): Promise<bigint> {
  if (!STAKING_ID) throw new Error('VITE_STAKING_ID not set');
  const v = await simulate(STAKING_ID, 'pending_points', [new Address(addr).toScVal()]);
  return BigInt(v);
}
export async function totalStaked(): Promise<bigint> {
  if (!STAKING_ID) return 0n;
  const v = await simulate(STAKING_ID, 'total_staked');
  return BigInt(v);
}

// ─── Writes ───
function i128Val(amount: bigint) {
  return nativeToScVal(amount, { type: 'i128' });
}

export async function stake(user: string, amount: bigint) {
  return sendTx(user, STAKING_ID!, 'stake', [
    new Address(user).toScVal(),
    i128Val(amount),
  ]);
}
export async function unstake(user: string, amount: bigint) {
  return sendTx(user, STAKING_ID!, 'unstake', [
    new Address(user).toScVal(),
    i128Val(amount),
  ]);
}
export async function claim(user: string) {
  return sendTx(user, STAKING_ID!, 'claim', [new Address(user).toScVal()]);
}
