import { createFileRoute } from "@tanstack/react-router";
import { getNftById } from "./-components/actions";

export const Route = createFileRoute("/nfts/$id")({
  component: RouteComponent,
  loader: ({ params: { id } }) => getNftById(id),
});

function RouteComponent() {
  const nft = Route.useLoaderData();

  return (
    <div>
      <img src={nft.imageUrl} />
    </div>
  );
}
