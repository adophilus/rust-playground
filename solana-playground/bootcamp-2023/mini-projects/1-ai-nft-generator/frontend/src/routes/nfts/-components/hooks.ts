import {
  useAppKitConnection,
  type Provider,
} from "@reown/appkit-adapter-solana/react";
import { useAppKitProvider } from "@reown/appkit/react";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import type { Nft } from "@nft-ai-generator/backend";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { useWallet } from "@solana/wallet-adapter-react";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { generateSigner, percentAmount } from "@metaplex-foundation/umi";
import { useMutation, type UseMutationOptions } from "@tanstack/react-query";

export const useMint = (nft: Nft, payload: UseMutationOptions) => {
  const { connection: solanaConnection } = useAppKitConnection();
  const solanaWallet = useWallet();
  const { walletProvider: solanaWalletProvider } =
    useAppKitProvider<Provider>("solana");

  const { mutate: mint, status } = useMutation({
    mutationFn: async () => {
      if (!solanaConnection) return;

      const umi = createUmi("https://api.devnet.solana.com")
        .use(mplTokenMetadata())
        .use(
          irysUploader({
            address: "https://devnet.irys.xyz",
          }),
        )
        .use(walletAdapterIdentity(solanaWallet));

      const metadata = {
        name: "NFT",
        description: "NFT description",
        symbol: "NFT",
        image: nft.image_url,
        external_url: "https://example.com/my-nft.json",
        attributes: [
          {
            trait_type: "trait1",
            value: "value1",
          },
          {
            trait_type: "trait2",
            value: "value2",
          },
        ],
        properties: {
          files: [
            {
              uri: nft.image_url,
              type: "image/png",
            },
          ],
          category: "image",
        },
      };

      const metadataUri = await umi.uploader.uploadJson(metadata);

      const nftSigner = generateSigner(umi);

      const tx = await createNft(umi, {
        mint: nftSigner,
        sellerFeeBasisPoints: percentAmount(5),
        name: metadata.name,
        uri: metadataUri,
      }).sendAndConfirm(umi);

      return tx;
    },
    ...payload,
  });

  return {
    mint,
    status,
  };
};
