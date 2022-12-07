import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from '@solana/web3.js';
import { MobileNumberIdentity } from '../target/types/mobile_number_identity';

import validatorKp from '../keys/validator.json';
import { assert } from 'chai';

describe('mobile-number-identity', () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace
		.MobileNumberIdentity as Program<MobileNumberIdentity>;

	const [globalPda] = findProgramAddressSync(
		[Buffer.from('global')],
		program.programId
	);

	const validator = Keypair.fromSecretKey(new Uint8Array(validatorKp));

	it('should register an identity', async () => {
		const randomPhoneNumber = Math.floor(Math.random() * 11) + '';
		const series = '2022340';
		const owner = Keypair.generate();

		const [identityPda] = findProgramAddressSync(
			[
				Buffer.from('identity'),
				Buffer.from(series),
				Buffer.from(randomPhoneNumber),
			],
			program.programId
		);

		const global = await program.account.global.fetch(globalPda);

		const accounts = {
			global: globalPda,
			identity: identityPda,
			owner: owner.publicKey,
			treasury: global.treasury,
			validator: validator.publicKey,
			systemProgram: SystemProgram.programId,
		};

		const connection = program.provider.connection;
		const latestBlockHash = await connection.getLatestBlockhash();
		const signature = await connection.requestAirdrop(
			owner.publicKey,
			LAMPORTS_PER_SOL
		);

		await connection.confirmTransaction({
			blockhash: latestBlockHash.blockhash,
			lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
			signature,
		});

		await program.methods
			.createIdentity({
				phoneNumber: randomPhoneNumber,
				series,
			})
			.accounts(accounts)
			.signers([owner, validator])
			.rpc();

		const result = await program.account.identity.fetch(identityPda);

		assert.ok(owner.publicKey.equals(result.owner));
	});
});
