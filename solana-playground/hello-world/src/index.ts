import { Keypair, PublicKey } from '@solana/web3.js'
import nacl from 'tweetnacl'

const keypair = Keypair.generate()

const message = 'This is a super secret message'
const messageBytes = new TextEncoder().encode(message)

const signature = nacl.sign.detached(messageBytes, keypair.secretKey)

const verification = nacl.sign.detached.verify(messageBytes, signature, keypair.publicKey.toBytes())

console.log(verification)