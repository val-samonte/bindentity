import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import {
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from '@solana/web3.js'
import { BN } from 'bn.js'
import { assert } from 'chai'
import { Bindentity } from '../target/types/bindentity'
import { airdrop } from '../scripts/utils'

describe('Provider Management', () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Bindentity as Program<Bindentity>

  const [globalPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('global')],
    program.programId,
  )

  const providerOwner = Keypair.generate()
  const validatorSigner = Keypair.generate()
  const user = Keypair.generate()
  const bindentityName = 'sample_' + Math.floor(Math.random() * 100_000_000)

  const [providerPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('provider'), Buffer.from(bindentityName, 'utf-8')],
    program.programId,
  )

  const [validatorPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('validator'),
      providerPda.toBytes(),
      validatorSigner.publicKey.toBytes(),
    ],
    program.programId,
  )

  before(async () => {
    const connection = program.provider.connection

    // provider creation fee is 1 SOL, so we need more than 1 SOL
    await airdrop(connection, providerOwner.publicKey)
    await airdrop(connection, providerOwner.publicKey)

    // send a few funds to the user who will avail the bindentity
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
  })

  it('should create a new bindentity provider', async () => {
    const global = await program.account.global.fetch(globalPda)

    try {
      await program.methods
        .createProvider({
          name: bindentityName,
          registrationFee: new BN(0),
          providerTreasury: providerOwner.publicKey,
        })
        .accounts({
          global: globalPda,
          owner: providerOwner.publicKey,
          provider: providerPda,
          treasury: global.treasury,
          systemProgram: SystemProgram.programId,
        })
        .signers([providerOwner])
        .rpc()
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }

    const provider = await program.account.provider.fetch(providerPda)

    assert.equal(provider.name, bindentityName)
  })

  it('should add a validator', async () => {
    try {
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
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  })

  it('should not allow the user to register when using a unpublished provider', async () => {
    // attempt to register
    const data = 'sample_user_id'
    const timestamp = new BN(Math.floor(new Date().getTime() / 1000))
    const [identityPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('bindie', 'utf-8'),
        Buffer.from(timestamp + '', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const [linkPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('link', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const global = await program.account.global.fetch(globalPda)
    const provider = await program.account.provider.fetch(providerPda)

    try {
      await program.methods
        .createBindie({
          data,
          encryptionCount: 0,
          registrationFee: new BN(0),
          timestamp,
        })
        .accounts({
          global: globalPda,
          bindie: identityPda,
          link: linkPda,
          owner: user.publicKey,
          provider: providerPda,
          providerTreasury: provider.treasury,
          signer: validatorSigner.publicKey,
          treasury: global.treasury,
          validator: validatorPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user, validatorSigner])
        .rpc()

      assert.ok(false)
    } catch (e) {
      // console.log(e)
    }
  })

  it('should be able to add metadata to a provider', async () => {
    const uri = `http://example.com`

    const [providerMetadataPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('provider_metadata', 'utf-8'), providerPda.toBytes()],
      program.programId,
    )

    try {
      await program.methods
        .createProviderMetadata({
          uri,
        })
        .accounts({
          authority: providerOwner.publicKey,
          provider: providerPda,
          providerMetadata: providerMetadataPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([providerOwner])
        .rpc()

      const metadata = await program.account.providerMetadata.fetch(
        providerMetadataPda,
      )

      assert.ok(metadata.uri === uri)
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  })

  it('should update provider config', async () => {
    try {
      await program.methods
        .updateProvider({
          published: true,
          authority: null,
          registrationFee: null,
          treasury: null,
          forSale: null,
          sellingPrice: null,
        })
        .accounts({
          authority: providerOwner.publicKey,
          provider: providerPda,
        })
        .signers([providerOwner])
        .rpc()
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  })

  it('should not allow the user to register when using a disabled validator', async () => {
    // disable validator
    await program.methods
      .updateValidator({
        flags: 0,
        close: null,
      })
      .accounts({
        authority: providerOwner.publicKey,
        provider: providerPda,
        validator: validatorPda,
      })
      .signers([providerOwner])
      .rpc()

    const data = 'sample_user_id'
    const timestamp = new BN(Math.floor(new Date().getTime() / 1000))
    const [identityPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('bindie', 'utf-8'),
        Buffer.from(timestamp + '', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const [linkPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('link', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const global = await program.account.global.fetch(globalPda)
    const provider = await program.account.provider.fetch(providerPda)

    try {
      await program.methods
        .createBindie({
          data,
          encryptionCount: 0,
          registrationFee: new BN(0),
          timestamp,
        })
        .accounts({
          global: globalPda,
          bindie: identityPda,
          link: linkPda,
          owner: user.publicKey,
          provider: providerPda,
          providerTreasury: provider.treasury,
          signer: validatorSigner.publicKey,
          treasury: global.treasury,
          validator: validatorPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user, validatorSigner])
        .rpc()

      assert.ok(false)
    } catch (e) {
      // console.log(e)
    }
  })

  it('should enable a validator', async () => {
    const validator = await program.account.validator.fetch(validatorPda)

    try {
      await program.methods
        .updateValidator({
          flags: validator.flags | 7,
          close: null,
        })
        .accounts({
          authority: providerOwner.publicKey,
          provider: providerPda,
          validator: validatorPda,
        })
        .signers([providerOwner])
        .rpc()
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  })

  it('should allow user to register to the new bindentity', async () => {
    const data = 'sample_user_id'
    const timestamp = new BN(Math.floor(new Date().getTime() / 1000))
    const [identityPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('bindie', 'utf-8'),
        Buffer.from(timestamp + '', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const [linkPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('link', 'utf-8'),
        providerPda.toBytes(),
        Buffer.from(data, 'utf-8'),
      ],
      program.programId,
    )

    const global = await program.account.global.fetch(globalPda)
    const provider = await program.account.provider.fetch(providerPda)

    try {
      await program.methods
        .createBindie({
          data,
          encryptionCount: 0,
          registrationFee: new BN(0),
          timestamp,
        })
        .accounts({
          global: globalPda,
          bindie: identityPda,
          link: linkPda,
          owner: user.publicKey,
          provider: providerPda,
          providerTreasury: provider.treasury,
          signer: validatorSigner.publicKey,
          treasury: global.treasury,
          validator: validatorPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user, validatorSigner])
        .rpc()
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  })
})
