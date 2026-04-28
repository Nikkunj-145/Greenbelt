import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  FREIGHTER_ID,
  ISupportedWallet,
} from '@creit.tech/stellar-wallets-kit';
import { NETWORK_PASSPHRASE } from './config';

export const kit = new StellarWalletsKit({
  network: WalletNetwork.TESTNET,
  selectedWalletId: FREIGHTER_ID,
  modules: allowAllModules(),
});

const STORAGE_KEY = 'greenbelt:walletId';

export async function openWalletPicker(): Promise<string> {
  // Always clear prior selection so the modal shows fresh & Freighter
  // re-prompts which account to use on connect.
  localStorage.removeItem(STORAGE_KEY);
  try {
    await kit.disconnect();
  } catch {}
  return new Promise((resolve, reject) => {
    kit
      .openModal({
        onWalletSelected: async (opt: ISupportedWallet) => {
          try {
            kit.setWallet(opt.id);
            localStorage.setItem(STORAGE_KEY, opt.id);
            const { address } = await kit.getAddress();
            resolve(address);
          } catch (e) {
            reject(e);
          }
        },
        onClosed: () => reject(new Error('Wallet selection cancelled')),
      })
      .catch(reject);
  });
}
export async function restoreWallet(): Promise<string | null> {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (!saved) return null;
  try {
    kit.setWallet(saved);
    const { address } = await kit.getAddress();
    return address;
  } catch {
    return null;
  }
}
export async function disconnect() {
  localStorage.removeItem(STORAGE_KEY);
  try {
    await kit.disconnect();
  } catch {}
}
export async function signXdr(xdr: string, address: string): Promise<string> {
  const { signedTxXdr } = await kit.signTransaction(xdr, {
    address,
    networkPassphrase: NETWORK_PASSPHRASE as WalletNetwork,
  });
  return signedTxXdr;
}
