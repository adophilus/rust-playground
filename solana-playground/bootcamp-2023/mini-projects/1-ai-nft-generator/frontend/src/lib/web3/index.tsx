import { type FunctionComponent, type ReactNode, useState } from "react";
import { Provider as EvmProvider } from "./evm";
import { Provider as SolanaProvider } from "./solana";
import { Button, StatefulButton } from "@/components/button";
import { ChevronDownIcon } from "lucide-react";
import { cn } from "../utils";
import solanaImage from "cryptocurrency-icons/32/white/sol.png";
import evmImage from "cryptocurrency-icons/32/white/eth.png";
import { useAppKit } from "@reown/appkit/react";

export const Provider: FunctionComponent<{ children: ReactNode }> = ({
  children,
}) => (
  <EvmProvider>
    <SolanaProvider>{children}</SolanaProvider>
  </EvmProvider>
);

type DropdownOption = {
  key: string;
  label: FunctionComponent;
  render: FunctionComponent<{
    close: () => void;
    isActive: boolean;
    activate: () => void;
  }>;
};

const Dropdown: FunctionComponent<{
  placeholder: string;
  options: DropdownOption[];
}> = ({ placeholder, options }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [active, setActive] = useState<string | null>(null);

  const toggleDropdown = () => setIsOpen((prev) => !prev);

  const ActiveLabel = options.find(({ key }) => key === active)?.label;

  return (
    <div className="relative flex flex-col gap-1">
      <div className="flex gap-1 w-full overflow-hidden">
        {active === null ? (
          <Button className="grow hover:scale-none">{placeholder}</Button>
        ) : (
          <ActiveLabel />
        )}
        <Button onClick={toggleDropdown} className="hover:scale-none shrink-0">
          <ChevronDownIcon className="size-5" />
        </Button>
      </div>
      <div
        className={cn(
          "w-full rounded-md overflow-clip absolute bottom-0 translate-y-[calc(100%+0.25rem)]",
          !isOpen && "hidden",
        )}
      >
        {options.map(({ key, render: Render }) => (
          <Render
            key={key}
            close={() => setIsOpen(false)}
            isActive={active === key}
            activate={() => setActive(key)}
          />
        ))}
      </div>
    </div>
  );
};

const buttonClasses =
  "hover:scale-none flex gap-2 items-center w-full rounded-none focus:outline-none";

export const ConnectWeb3Button = () => {
  return (
    <Dropdown
      placeholder="Select network"
      options={[
        {
          key: "evm",
          label: () => {
            const { open } = useAppKit();

            const connected = false;

            return (
              <Button className="grow" onClick={() => open()}>
                {connected ? (
                  <span className="w-full">connected</span>
                ) : (
                  <span className="flex items-center gap-2">
                    <img src={evmImage} className="size-5" alt="EVM" />
                    EVM
                  </span>
                )}
              </Button>
            );
          },
          render: ({ isActive, activate, close }) => {
            return (
              <Button
                type="button"
                className={buttonClasses}
                onClick={() => {
                  activate();
                  close();
                }}
              >
                <img src={evmImage} className="size-5" alt="Solana" />
                EVM
              </Button>
            );
          },
        },
        {
          key: "solana",
          label: () => {
            const { open } = useAppKit();

            return (
              <Button
                className="flex items-center gap-2 grow"
                onClick={() => open()}
              >
                <img src={solanaImage} className="size-5" alt="Solana" />
                Solana
              </Button>
            );
          },
          render: ({ isActive, activate, close }) => (
            <Button
              type="button"
              className={buttonClasses}
              onClick={() => {
                activate();
                close();
              }}
            >
              <img src={solanaImage} className="size-5" alt="Solana" />
              Solana
            </Button>
          ),
        },
      ]}
    />
  );
};
