import { createAppKit } from "@reown/appkit/react";

import { WagmiProvider } from "wagmi";
import { baseSepolia, sepolia, solanaDevnet } from "@reown/appkit/networks";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { env } from "@/env";
import type { FunctionComponent, ReactNode } from "react";
import { SolanaAdapter } from "@reown/appkit-adapter-solana/react";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";

const solanaWeb3JsAdapter = new SolanaAdapter({
  wallets: [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
});

const queryClient = new QueryClient();

const projectId = env.REOWN_PROJECT_ID;

const metadata = {
  name: "AI NFT Generator",
  description: "AI NFT Generator",
  url: "https://example.com", // origin must match your domain & subdomain
  icons: ["https://avatars.githubusercontent.com/u/179229932"],
};

const networks = [sepolia, baseSepolia, solanaDevnet];

const wagmiAdapter = new WagmiAdapter({
  networks,
  projectId,
  ssr: true,
});

createAppKit({
  adapters: [wagmiAdapter, solanaWeb3JsAdapter],
  networks,
  projectId,
  metadata,
});

export const Provider: FunctionComponent<{ children: ReactNode }> = ({
  children,
}) => {
  return (
    <WagmiProvider config={wagmiAdapter.wagmiConfig}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </WagmiProvider>
  );
};
