import { createFileRoute } from "@tanstack/react-router";
import { getNftById } from "./-components/actions";
import { useRef, type FunctionComponent } from "react";
import type { Nft } from "@nft-ai-generator/backend";
import { StatefulButton } from "../../components/button";
import { ConnectButton } from "@/lib/web3";
import { useAppKitAccount, useAppKitProvider } from "@reown/appkit/react";
import { BadgeCheckIcon } from "lucide-react";
import { useMint } from "./-components/hooks";
import { toast } from "sonner";

export const Route = createFileRoute("/nfts/$id")({
  component: RouteComponent,
  loader: ({ params: { id } }) => getNftById(id),
});

const MintButton: FunctionComponent<{ nft: Nft }> = ({ nft }) => {
  const { status } = useAppKitAccount();
  const toastId = useRef<string | number>();
  const isMinting = false;

  const { mint } = useMint(nft, {
    onSuccess: (tx) => {
      console.log(tx);
      toast.dismiss(toastId.current);
      toast.success("Minted");
    },
    onError: (err) => {
      console.log(err);
      toast.dismiss(toastId.current);
      toast.error(err.message);
    },
  });

  const isDisabled = status !== "connected";

  const onClick = async () => {
    toastId.current = toast.loading("Minting...");
    mint();
  };

  return (
    <StatefulButton
      isLoading={isMinting}
      variant="secondary"
      disabled={isDisabled}
      onClick={onClick}
    >
      <span className="inline-flex items-center gap-1">
        <BadgeCheckIcon className="size-4 stroke-emerald-500" />
        Mint
      </span>
    </StatefulButton>
  );
};

const NftCard: FunctionComponent<{ nft: Nft }> = ({ nft }) => {
  return (
    <div className="w-[250px] border border-2 shadow-sm border-gray-300 rounded-md p-2 flex gap-2 flex-col">
      <div className="relative">
        <img
          src={nft.image_url}
          alt={nft.prompt}
          className="w-full h-[200px] object-cover rounded-md"
        />
        <div className="inset-0 absolute flex justify-end items-start p-1 bg-black/40 rounded-md">
          <MintButton nft={nft} />
        </div>
      </div>
      <ConnectButton />
    </div>
  );
};

function RouteComponent() {
  const nft = Route.useLoaderData();

  return (
    <div className="h-screen flex items-center justify-center">
      <NftCard nft={nft} />
    </div>
  );
}
