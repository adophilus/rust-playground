import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Voting } from '../target/types/voting'
import IDL from '../target/idl/voting.json' assert { type: 'json'}

describe('voting', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Voting as Program<Voting>

  const votingKeypair = Keypair.generate()

  it('Initialize Voting', async () => {
  })
})
