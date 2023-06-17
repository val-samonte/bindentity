import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import authorityJSON from '../keys/authority.json'

const authority = Keypair.fromSecretKey(new Uint8Array(authorityJSON))

export interface AnchorWallet {
  publicKey: PublicKey
  signTransaction(transaction: Transaction): Promise<Transaction>
  signAllTransactions(transactions: Transaction[]): Promise<Transaction[]>
}

export class KeypairWallet implements AnchorWallet {
  constructor(readonly payer: Keypair) {}

  async signTransaction(tx: Transaction): Promise<Transaction> {
    tx.partialSign(this.payer)
    return tx
  }

  async signAllTransactions(txs: Transaction[]): Promise<Transaction[]> {
    return txs.map((tx) => {
      tx.partialSign(this.payer)
      return tx
    })
  }

  get publicKey(): PublicKey {
    return this.payer.publicKey
  }
}

export async function airdrop(
  connection: Connection,
  publicKey: PublicKey,
  amount = LAMPORTS_PER_SOL,
) {
  try {
    const latestBlockHash = await connection.getLatestBlockhash()
    const signature = await connection.requestAirdrop(publicKey, amount)

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature,
    })
  } catch (e) {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: publicKey,
        lamports: amount,
      }),
    )
    await sendAndConfirmTransaction(connection, transaction, [authority])
  }
}
