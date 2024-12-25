import { backendClient } from "../../../lib/http";

export const getNftById = async (id: string) => {
  const res = await backendClient.api.nfts[":id"].$get({ param: { id } });
  if (res.status !== 200) {
    const json = await res.json();
    throw new Error(json.error);
  }

  return res.json();
};
