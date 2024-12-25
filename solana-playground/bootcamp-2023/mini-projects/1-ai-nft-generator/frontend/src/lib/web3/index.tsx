import type { FunctionComponent, ReactNode } from "react";
import { Provider as EvmProvider } from "./evm";
import { Provider as SolanaProvider } from "./solana";

export const Provider: FunctionComponent<{ children: ReactNode }> = ({
  children,
}) => (
  <EvmProvider>
    <SolanaProvider>{children}</SolanaProvider>
  </EvmProvider>
);

export const ConnectButton = () => null
