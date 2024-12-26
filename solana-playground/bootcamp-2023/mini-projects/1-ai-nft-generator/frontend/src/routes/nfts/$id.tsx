import { createFileRoute } from "@tanstack/react-router";
import { getNftById } from "./-components/actions";
import type { FunctionComponent } from "react";
import type { Nft } from "@nft-ai-generator/backend";
import { StatefulButton } from "../../components/button";
import { ConnectWeb3Button } from "@/lib/web3";

export const Route = createFileRoute("/nfts/$id")({
  component: RouteComponent,
  loader: ({ params: { id } }) => getNftById(id),
});

const NftCard: FunctionComponent<{ nft: Nft }> = ({ nft }) => {
  const isMinting = false;

  return (
    <div className="w-[250px] border border-2 shadow-sm border-gray-300 rounded-md p-2 flex gap-2 flex-col">
      <div className="relative">
        <img
          src={nft.image_url}
          alt={nft.prompt}
          className="w-full h-[200px] object-cover rounded-md"
        />
        <div className="inset-0 absolute flex justify-end items-start p-1 bg-black/40 rounded-md">
          <StatefulButton isLoading={isMinting} variant="secondary">
            Mint
          </StatefulButton>
        </div>
      </div>
      <ConnectWeb3Button />
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
