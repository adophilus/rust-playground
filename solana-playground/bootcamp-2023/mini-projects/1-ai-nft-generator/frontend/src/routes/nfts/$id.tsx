import { createFileRoute } from "@tanstack/react-router";
import { getNftById } from "./-components/actions";
import type { FunctionComponent } from "react";
import type { Nft } from "@nft-ai-generator/backend";
import { StatefulButton } from "../../components/button";

export const Route = createFileRoute("/nfts/$id")({
  component: RouteComponent,
  loader: ({ params: { id } }) => getNftById(id),
});

const NftCard: FunctionComponent<{ nft: Nft }> = ({ nft }) => {
  const isMinting = false;

  return (
    <div className="w-[250px] border border-2 shadow-sm border-gray-300 rounded-md p-2 flex gap-2 flex-col">
      <img
        src={nft.image_url}
        alt={nft.prompt}
        className="w-full h-[200px] object-cover rounded-md"
      />
      <StatefulButton isLoading={isMinting}>Mint</StatefulButton>
    </div>
  );
};

function RouteComponent() {
  const nft = Route.useLoaderData();

  return (
    <div className="h-full flex items-center justify-center">
      <NftCard nft={nft} />
    </div>
  );
}
