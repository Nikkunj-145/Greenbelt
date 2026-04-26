/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_TOKEN_ID?: string;
  readonly VITE_STAKING_ID?: string;
  readonly VITE_NETWORK_PASSPHRASE?: string;
  readonly VITE_RPC_URL?: string;
}
interface ImportMeta { readonly env: ImportMetaEnv; }
