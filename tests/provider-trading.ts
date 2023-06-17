import * as anchor from '@project-serum/anchor'
import { AnchorError, Program } from '@project-serum/anchor'
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey'
import {
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from '@solana/web3.js'
import { BN } from 'bn.js'
import { assert } from 'chai'
import { Bindentity } from '../target/types/bindentity'
import { airdrop } from '../scripts/utils'

describe('Provider Trading', () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Bindentity as Program<Bindentity>

  const [globalPda] = findProgramAddressSync(
    [Buffer.from('global')],
    program.programId,
  )

  const providerOwner = Keypair.generate()
  const validatorSigner = Keypair.generate()
  const user = Keypair.generate()
  const bindentityName = 'sample_' + Math.floor(Math.random() * 100_000_000)

  const [providerPda] = findProgramAddressSync(
    [Buffer.from('provider'), Buffer.from(bindentityName, 'utf-8')],
    program.programId,
  )

  const [validatorPda] = findProgramAddressSync(
    [
      Buffer.from('validator'),
      providerPda.toBytes(),
      validatorSigner.publicKey.toBytes(),
    ],
    program.programId,
  )

  let global

  before(async () => {
    const connection = program.provider.connection

    // provider creation fee is 1 SOL, so we need more than 1 SOL
    await airdrop(connection, providerOwner.publicKey)
    await airdrop(connection, providerOwner.publicKey)

    // send a few funds to the user who will buy the provider
    try {
      await sendAndConfirmTransaction(
        connection,
        new Transaction().add(
          SystemProgram.transfer({
            fromPubkey: providerOwner.publicKey,
            toPubkey: user.publicKey,
            lamports: LAMPORTS_PER_SOL / 10,
          }),
        ),
        [providerOwner],
      )
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }

    global = await program.account.global.fetch(globalPda)

    // create provider

    await program.methods
      .createProvider({
        name: bindentityName,
        providerTreasury: providerOwner.publicKey,
        registrationFee: new BN(0),
      })
      .accounts({
        owner: providerOwner.publicKey,
        provider: providerPda,
        treasury: global.treasury,
        global: globalPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([providerOwner])
      .rpc()
  })

  it('should not allow sell if the provider is not for sale', async () => {
    // attempt to buy the provider
    try {
      await program.methods
        .buyProvider({
          providerTreasury: user.publicKey,
          registrationFee: new BN(0),
        })
        .accounts({
          seller: providerOwner.publicKey,
          buyer: user.publicKey,
          provider: providerPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()

      assert.ok(false)
    } catch (err) {
      const e = err as AnchorError
      assert.strictEqual(e.error.errorCode.code, 'SellingNotAllowed')
    }
  })

  it('should not allow listing if the provider has a validator', async () => {
    // create validator
    await program.methods
      .createValidator({
        enabled: true,
        signer: validatorSigner.publicKey,
      })
      .accounts({
        authority: providerOwner.publicKey,
        provider: providerPda,
        validator: validatorPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([providerOwner])
      .rpc()

    try {
      // attempt to list for sale
      await program.methods
        .updateProvider({
          forSale: true,
          published: null,
          sellingPrice: new BN(LAMPORTS_PER_SOL / 1000),
          authority: null,
          registrationFee: null,
          treasury: null,
        })
        .accounts({
          authority: providerOwner.publicKey,
          provider: providerPda,
        })
        .signers([providerOwner])
        .rpc()

      assert.ok(false)
    } catch (err) {
      const e = err as AnchorError
      assert.strictEqual(e.error.errorCode.code, 'SellingNotAllowed')
    }
  })

  it('should allow listing of the provider if it has no validator', async () => {
    // remove validator
    await program.methods
      .updateValidator({
        close: true,
        flags: null,
      })
      .accounts({
        authority: providerOwner.publicKey,
        provider: providerPda,
        validator: validatorPda,
      })
      .signers([providerOwner])
      .rpc()

    // list the provider for sale
    await program.methods
      .updateProvider({
        forSale: true,
        published: null,
        sellingPrice: new BN(LAMPORTS_PER_SOL / 1000),
        authority: null,
        registrationFee: null,
        treasury: null,
      })
      .accounts({
        authority: providerOwner.publicKey,
        provider: providerPda,
      })
      .signers([providerOwner])
      .rpc()

    const provider = await program.account.provider.fetch(providerPda)

    // check sell flag
    assert.ok((provider.flags & 8) === 8)
  })

  it('should allow the user to buy the listed provider', async () => {
    // buy the provider
    await program.methods
      .buyProvider({
        providerTreasury: user.publicKey,
        registrationFee: new BN(0),
      })
      .accounts({
        seller: providerOwner.publicKey,
        buyer: user.publicKey,
        provider: providerPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc()

    const provider = await program.account.provider.fetch(providerPda)

    assert.ok(provider.authority.equals(user.publicKey))
  })
})
