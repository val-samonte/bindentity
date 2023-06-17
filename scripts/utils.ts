import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  VersionedTransaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js'
import authorityJSON from '../keys/authority.json'
import { Wallet } from '@coral-xyz/anchor'
import { isVersionedTransaction } from '@coral-xyz/anchor/dist/cjs/utils/common'

const authority = Keypair.fromSecretKey(new Uint8Array(authorityJSON))

export class KeypairWallet implements Wallet {
  constructor(readonly payer: Keypair) {}

  async signTransaction<T extends Transaction | VersionedTransaction>(
    tx: T,
  ): Promise<T> {
    if (isVersionedTransaction(tx)) {
      tx.sign([this.payer])
    } else {
      tx.partialSign(this.payer)
    }

    return tx
  }

  async signAllTransactions<T extends Transaction | VersionedTransaction>(
    txs: T[],
  ): Promise<T[]> {
    return txs.map((t) => {
      if (isVersionedTransaction(t)) {
        t.sign([this.payer])
      } else {
        t.partialSign(this.payer)
      }
      return t
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
