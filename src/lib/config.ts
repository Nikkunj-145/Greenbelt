export const TOKEN_ID = import.meta.env.VITE_TOKEN_ID as string | undefined;
export const STAKING_ID = import.meta.env.VITE_STAKING_ID as string | undefined;
export const NETWORK_PASSPHRASE =
  (import.meta.env.VITE_NETWORK_PASSPHRASE as string) || 'Test SDF Network ; September 2015';
export const RPC_URL =
  (import.meta.env.VITE_RPC_URL as string) || 'https://soroban-testnet.stellar.org';

export const TOKEN_DECIMALS = 7;
export const TOKEN_SYMBOL = 'STK';
