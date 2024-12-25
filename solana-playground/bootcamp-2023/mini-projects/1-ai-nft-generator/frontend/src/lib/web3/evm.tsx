import type { FunctionComponent, ReactNode } from "react";
import { http, createConfig, WagmiProvider } from "wagmi";
import { sepolia, baseSepolia } from "wagmi/chains";
import "@rainbow-me/rainbowkit/styles.css";
import { RainbowKitProvider } from "@rainbow-me/rainbowkit";

export const config = createConfig({
  chains: [sepolia, baseSepolia],
  connectors: [],
  transports: {
    [sepolia.id]: http(),
    [baseSepolia.id]: http(),
  },
});

export const Provider: FunctionComponent<{ children: ReactNode }> = ({
  children,
}) => (
  <WagmiProvider config={config}>
    <RainbowKitProvider>{children}</RainbowKitProvider>
  </WagmiProvider>
);
