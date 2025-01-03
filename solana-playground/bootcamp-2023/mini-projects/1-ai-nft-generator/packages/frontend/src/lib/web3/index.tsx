import type { FunctionComponent, ReactNode } from "react";
import { Provider as Web3Provider } from "./provider";
import "@reown/appkit-wallet-button/react";
import { Button } from "@/components/button";
import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import truncateMiddle from "truncate-middle";
import { Loader2Icon } from "lucide-react";

export const Provider: FunctionComponent<{ children: ReactNode }> = ({
  children,
}) => <Web3Provider>{children}</Web3Provider>;

export const ConnectButton = () => {
  const { open } = useAppKit();
  const { address, status } = useAppKitAccount();

  return (
    <Button onClick={() => open()}>
      {status === "connected" && truncateMiddle(address as string, 4, 4, "...")}
      {status === "connecting" ||
        (status === "reconnecting" && (
          <Loader2Icon className="animate-spin size-5" />
        ))}
      {status === "disconnected" && "Connect Wallet"}
    </Button>
  );
};
