import { AnchorProvider, BN, Program } from '@project-serum/anchor'
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey'
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from '@solana/web3.js'

import idl from '../target/idl/bindentity.json'
import { Bindentity } from '../target/types/bindentity'

import programKp from '../target/deploy/bindentity-keypair.json'
import authorityKp from '../keys/authority.json'
import validatorKp from '../keys/validator.json'
import treasuryKp from '../keys/treasury.json'
import { KeypairWallet } from './utils'

const { publicKey: programId } = Keypair.fromSecretKey(
  new Uint8Array(programKp),
)
const authority = Keypair.fromSecretKey(new Uint8Array(authorityKp))
const validator = Keypair.fromSecretKey(new Uint8Array(validatorKp))
const treasury = Keypair.fromSecretKey(new Uint8Array(treasuryKp))

const program = new Program<Bindentity>(
  idl as unknown as Bindentity,
  programId,
  new AnchorProvider(
    AnchorProvider.env().connection,
    new KeypairWallet(authority),
    {},
  ),
)

const [programDataPda] = findProgramAddressSync(
  [programId.toBytes()],
  new PublicKey('BPFLoaderUpgradeab1e11111111111111111111111'),
)

const [globalPda] = findProgramAddressSync([Buffer.from('global')], programId)

const officialProviders = [
  'phone',
  'email',
  'provider',
  'mythrilsoft',
  'bindentity',
  'bindie',
  'web3triad',
  'ethereum',
  'twitter',
  'discord',
]

const init = async () => {
  // initialize global config, set provider_creation_fee to 0
  const existingConfig = await program.account.global.fetchNullable(globalPda)

  if (!existingConfig) {
    try {
      const accounts = {
        global: globalPda,
        authority: authority.publicKey,
        program: programId,
        programData: programDataPda,
        systemProgram: SystemProgram.programId,
      }

      console.log(JSON.stringify(accounts, null, 2))

      await program.methods
        .initialize({
          treasury: treasury.publicKey,
          providerCreationFee: new BN(0),
          serviceFee: new BN(0),
        })
        .accounts(accounts)
        .rpc()

      console.log('Global config initialized')
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  }

  const registrationFee = new BN(LAMPORTS_PER_SOL / 10)

  // create initial providers
  await Promise.all(
    officialProviders.map(async (bindentityName) => {
      const isPublished = ['phone', 'email', 'provider'].includes(
        bindentityName,
      )
      const tx = new Transaction()

      const [providerPda] = findProgramAddressSync(
        [
          Buffer.from('provider', 'utf-8'),
          Buffer.from(bindentityName, 'utf-8'),
        ],
        program.programId,
      )

      const providerIx = await program.methods
        .createProvider({
          name: bindentityName,
          providerTreasury: treasury.publicKey,
          registrationFee,
        })
        .accounts({
          treasury: treasury.publicKey,
          global: globalPda,
          owner: authority.publicKey,
          provider: providerPda,
          systemProgram: SystemProgram.programId,
        })
        .instruction()

      tx.add(providerIx)

      if (isPublished) {
        const [providerMetadataPda] = findProgramAddressSync(
          [Buffer.from('provider_metadata', 'utf-8'), providerPda.toBytes()],
          program.programId,
        )

        const metadataIx = await program.methods
          .createProviderMetadata({
            uri: `https://shdw-drive.genesysgo.net/EQUAMGwdZNwhuZxXVFeVmxVYd3ZWMhL1TYFoM1WScLgQ/${bindentityName}.json`,
          })
          .accounts({
            authority: authority.publicKey,
            provider: providerPda,
            providerMetadata: providerMetadataPda,
            systemProgram: SystemProgram.programId,
          })
          .instruction()

        tx.add(metadataIx)

        const publishIx = await program.methods
          .updateProvider({
            published: true,
            authority: null,
            forSale: null,
            registrationFee: null,
            sellingPrice: null,
            treasury: null,
          })
          .accounts({
            authority: authority.publicKey,
            provider: providerPda,
          })
          .instruction()

        tx.add(publishIx)

        // create validators for each published providers
        const [validatorPda] = findProgramAddressSync(
          [
            Buffer.from('validator', 'utf-8'),
            providerPda.toBytes(),
            validator.publicKey.toBytes(),
          ],
          program.programId,
        )

        const validatorIx = await program.methods
          .createValidator({
            enabled: true,
            signer: validator.publicKey,
          })
          .accounts({
            authority: authority.publicKey,
            provider: providerPda,
            validator: validatorPda,
            systemProgram: SystemProgram.programId,
          })
          .instruction()

        tx.add(validatorIx)
      }

      if (program.provider.sendAndConfirm) {
        return program.provider.sendAndConfirm(tx, [])
      }

      return null
    }),
  )

  // make authority a verified provider
  const timestamp = new BN(Math.floor(new Date().getTime() / 1000))

  const [verifierPda] = findProgramAddressSync(
    [Buffer.from('provider', 'utf-8'), Buffer.from('provider', 'utf-8')],
    program.programId,
  )

  const [validatorPda] = findProgramAddressSync(
    [
      Buffer.from('validator', 'utf-8'),
      verifierPda.toBytes(),
      validator.publicKey.toBytes(),
    ],
    program.programId,
  )

  const [bindiePda] = findProgramAddressSync(
    [
      Buffer.from('bindie', 'utf-8'),
      Buffer.from(timestamp + '', 'utf-8'),
      verifierPda.toBytes(),
      Buffer.from(authority.publicKey.toBase58().substring(0, 32), 'utf-8'),
    ],
    program.programId,
  )

  const [linkPda] = findProgramAddressSync(
    [
      Buffer.from('link', 'utf-8'),
      verifierPda.toBytes(),
      Buffer.from(authority.publicKey.toBase58().substring(0, 32), 'utf-8'),
    ],
    program.programId,
  )

  const existingLink = await program.account.link.fetchNullable(linkPda)

  if (!existingLink) {
    try {
      await program.methods
        .createBindie({
          encryptionCount: 0,
          data: authority.publicKey.toBase58(),
          registrationFee: new BN(0),
          timestamp,
        })
        .accounts({
          bindie: bindiePda,
          link: linkPda,
          provider: verifierPda,
          providerTreasury: treasury.publicKey,
          validator: validatorPda,
          signer: validator.publicKey,
          owner: authority.publicKey,
          global: globalPda,
          treasury: treasury.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([validator])
        .rpc()
    } catch (e) {
      console.log(e)
      throw new Error(e)
    }
  }

  // verify each initial providers (including the verifier itself)
  try {
    await Promise.all(
      officialProviders.map(async (bindentityName) => {
        const [providerPda] = findProgramAddressSync(
          [
            Buffer.from('provider', 'utf-8'),
            Buffer.from(bindentityName, 'utf-8'),
          ],
          program.programId,
        )

        return program.methods
          .verifyProvider({
            data: authority.publicKey.toBase58(),
          })
          .accounts({
            targetProvider: providerPda,
            owner: authority.publicKey,
            ownerBindie: existingLink ? existingLink.bindie : bindiePda,
            ownerLink: linkPda,
            verifierProvider: verifierPda,
            validator: validatorPda,
            signer: validator.publicKey,
          })
          .signers([validator])
          .rpc()
      }),
    )
  } catch (e) {
    console.log(e)
  }

  // update fees
  await program.methods
    .updateConfig({
      providerCreationFee: new BN(LAMPORTS_PER_SOL),
      serviceFee: new BN(LAMPORTS_PER_SOL / 100),
      authority: null,
      treasury: null,
    })
    .accounts({
      authority: authority.publicKey,
      global: globalPda,
    })
    .rpc()

  const providers = await program.account.provider.all()
  console.log('Registered providers:', JSON.stringify(providers, null, 2))
}

init()
