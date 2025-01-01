import {
  useAppKitConnection,
  type Provider,
} from "@reown/appkit-adapter-solana/react";
import { useAppKitProvider } from "@reown/appkit/react";
import type { Nft } from "@nft-ai-generator/backend";
import {
  // PROGRAM_ID as MPL_TOKEN_METADATA_PROGRAM_ID,
  // createCreateMetadataAccountV3Instruction,
  MPL_TOKEN_METADATA_PROGRAM_ID,
  createMetadataAccountV3,
} from "@metaplex-foundation/mpl-token-metadata";
import { useMutation, type UseMutationOptions } from "@tanstack/react-query";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  createSetAuthorityInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "assertate";
import { createDefaultProgramRepository } from "@metaplex-foundation/umi-program-repository";
import { createUmi } from "@metaplex-foundation/umi";
import { createWeb3JsEddsa } from "@metaplex-foundation/umi-eddsa-web3js";

export const useMint = (nft: Nft, payload: UseMutationOptions) => {
  const { connection } = useAppKitConnection();
  const { walletProvider } = useAppKitProvider<Provider>("solana");

  const { mutate: mint, status } = useMutation({
    mutationFn: async () => {
      assert(connection, "Connection is required");
      assert(walletProvider, "Wallet provider is required");
      assert(
        walletProvider.publicKey,
        "Wallet provider public key is required",
      );

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

      const mintKeypair = Keypair.generate();

      console.log("Calculating minimum balance for mint account...");
      const lamportsForMintAccount =
        await getMinimumBalanceForRentExemptMint(connection);

      console.log("Creating mint account instruction...");
      const createMintAccountInstruction = SystemProgram.createAccount({
        fromPubkey: walletProvider.publicKey, // payer
        newAccountPubkey: mintKeypair.publicKey, // mint account address
        space: MINT_SIZE, // space (in bytes) required by mint account
        lamports: lamportsForMintAccount, // lamports to transfer to mint account
        programId: TOKEN_PROGRAM_ID, // new program owner
        // programId: SystemProgram.programId,
      });

      console.log("initializing mint account instruction...");
      // 2) Instruction to invoke Token Program to initialize mint account
      const initializeMintInstruction = createInitializeMint2Instruction(
        mintKeypair.publicKey, // mint address
        0, // decimals of mint (0 for NFTs)
        walletProvider.publicKey, // mint authority
        walletProvider.publicKey, // freeze authority
      );

      console.log("Creating associated token account instruction...");
      // Derive associated token account address from mint address and token account owner
      // This address is a PDA (Program Derived Address) and is generated deterministically
      const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
        mintKeypair.publicKey, // mint address
        walletProvider.publicKey, // token account owner
      );
      console.log(
        "Associated token account address:",
        associatedTokenAccountAddress.toBase58(),
      );

      console.log("Creating create associated token account instruction...");
      // 3) Instruction to invoke Associated Token Program to create associated token account
      // The Associated Token Program invokes the Token Program to create the token account with a PDA as the address of the token account
      const createTokenAccountInstruction =
        createAssociatedTokenAccountInstruction(
          walletProvider.publicKey, // payer
          associatedTokenAccountAddress, // associated token account address
          walletProvider.publicKey, // owner
          mintKeypair.publicKey, // mint address
        );

      console.log("Creating mint token instruction...");
      // 4) Instruction to invoke Token Program to mint 1 token to associated token account
      const mintTokenInstruction = createMintToInstruction(
        mintKeypair.publicKey, // mint address
        associatedTokenAccountAddress, // destination
        walletProvider.publicKey, // mint authority
        1, // amount
      );

      console.log("Deriving metadata account address...");
      // Derive the Metadata account address
      const [metadataAccountAddress] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"), // hard-coded string "metadata"
          new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID.toString()).toBuffer(), // metadata program address
          new PublicKey(mintKeypair.publicKey.toString()).toBuffer(), // mint address
        ],
        new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID.toString()), // metadata program address
      );
      console.log("Metadata account address:", metadataAccountAddress);
      console.log(
        "Metadata account address:",
        metadataAccountAddress.toBase58(),
      );

      const umi = createUmi();

      const myPublicKey = new PublicKey(
        walletProvider.publicKey.toString(),
      ).toBase58();

      console.log("Creating create metadata account instruction...");
      // 5) Instruction invoke Metaplex Token Metadata Program to create the Metadata account
      const mplCreateMetadataAccountInstruction = createMetadataAccountV3(
        {
          // metadata: metadataAccountAddress, // metadata account address
          // mint: mintKeypair.publicKey, // mint address
          // mintAuthority: walletProvider.publicKey, // authority to mint tokens
          // payer: walletProvider, // payer
          // updateAuthority: walletProvider.publicKey, // authority to update metadata account

          eddsa: createWeb3JsEddsa(),
          identity: walletProvider.publicKey,
          // identity: {
          //   ...walletProvider,
          //   publicKey: myPublicKey,
          // },
          // payer: {
          //   ...walletProvider,
          //   publicKey: myPublicKey,
          // },
          payer: walletProvider.publicKey,
          programs: createDefaultProgramRepository({
            rpc: umi.rpc,
          }),
        },
        {
          metadata: metadataAccountAddress,
          mint: mintKeypair.publicKey,
          mintAuthority: myPublicKey,
          data: {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: "https://google.com",
            sellerFeeBasisPoints: 500,
            creators: null,
            collection: null,
            uses: null,
          },
          isMutable: true,
          collectionDetails: null,

          // createMetadataAccountArgsV3: {
          //   data: {
          //     creators: null, // creators of the NFT (optional)
          //     name: metadata.name, // on-chain name
          //     symbol: metadata.symbol, // on-chain symbol
          //     uri: nft.metadata_uri ?? "https://google.com", // off-chain metadata
          //     sellerFeeBasisPoints: 0, // royalty fee
          //     collection: null, // collection the NFT belongs to (optional)
          //     uses: null, // uses (optional)
          //   },
          //   collectionDetails: null, // collection details (optional)
          //   isMutable: false, // whether the metadata can be changed
          // },
        },
      )
        // .useV0()
        .getInstructions()[0];
      console.log(mplCreateMetadataAccountInstruction);

      console.log("Creating set authority instruction...");
      // 6) Instruction to invoke Token Program to set mint authority to null
      const setAuthorityInstruction = createSetAuthorityInstruction(
        mintKeypair.publicKey, // mint address
        walletProvider.publicKey, // current authority (mint authority)
        0, // authority type (mint authority)
        null, // new authority
      );

      const blockhash = await connection
        .getLatestBlockhash()
        .then((res) => res.blockhash);

      console.log("Creating transaction message...");
      const message = new TransactionMessage({
        // payerKey: new PublicKey(walletProvider.publicKey.toString()),
        payerKey: walletProvider.publicKey,
        recentBlockhash: blockhash,
        instructions: [
          // 1) Create mint account
          createMintAccountInstruction,

          // 2) Initialize mint account
          initializeMintInstruction,

          // 3) Create associated token account
          createTokenAccountInstruction,

          // 4) Mint 1 token to associated token account
          mintTokenInstruction,

          // 5) Create metadata account
          // createMetadataInstruction,
          new TransactionInstruction({
            keys: mplCreateMetadataAccountInstruction.keys.map((key) => ({
              ...key,
              pubkey: new PublicKey(key.pubkey),
            })),
            programId: new PublicKey(
              mplCreateMetadataAccountInstruction.programId,
            ),
            data: mplCreateMetadataAccountInstruction.data,
          }),

          // 6) Set mint authority to null
          // setAuthorityInstruction,
        ],
      }).compileToV0Message();

      console.log("Creating transaction...");
      // Create new transaction and add instructions
      const transaction = new VersionedTransaction(message);

      console.log("Sending transaction...");
      try {
        transaction.sign([mintKeypair]);
        const signature = await walletProvider.sendTransaction(
          transaction,
          connection,
        );
        return signature;
      } catch (err) {
        console.log(err);
        throw new Error("Failed to mint NFT");
      }
    },
    ...payload,
  });

  return {
    mint,
    status,
  };
};
