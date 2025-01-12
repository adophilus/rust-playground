import Arweave from "arweave";
import type { StorageService, StorageServiceError } from "./types";
import Result, { ok, err } from "true-myth/result";
import assert from "node:assert";

const generateWallet = async (arweave: Arweave) => arweave.wallets.generate();
type Wallet = Awaited<ReturnType<typeof generateWallet>>;

class ArweaveStorageService implements StorageService {
  private declare arweave: Arweave;
  private declare wallet: Wallet;

  constructor() {
    this.arweave = Arweave.init({
      host: "arweave.net",
      port: 443,
      protocol: "https",
    });
    generateWallet(this.arweave).then((wallet) => {
      this.wallet = wallet;
    });
  }

  async uploadFile(file: File): Promise<Result<string, StorageServiceError>> {
    try {
      const tx = await this.arweave.createTransaction(
        {
          data: await file.arrayBuffer(),
        },
        this.wallet,
      );
      tx.addTag("Content-Type", file.type);

      await this.arweave.transactions.sign(tx, this.wallet);

      const uploader = await this.arweave.transactions.getUploader(tx);

      while (!uploader.isComplete) {
        await uploader.uploadChunk();
      }

      const { status, confirmed } = await this.arweave.transactions.getStatus(
        tx.id,
      );

      if (status !== 200) return err("UPLOAD_FAILED");
      assert(confirmed !== null, "Invariant reached");

      if (confirmed.number_of_confirmations)
        return ok(`https://arweave.net/${tx.id}`);

      return err("UPLOAD_FAILED");
    } catch (error) {
      console.warn("Failed to upload image:", error);
      return err("UPLOAD_FAILED");
    }
  }
}

export { ArweaveStorageService };
