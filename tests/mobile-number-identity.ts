import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { Keypair, SystemProgram } from '@solana/web3.js';
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
		const owner = Keypair.generate();

		const [identityPda] = findProgramAddressSync(
			[Buffer.from('identity'), Buffer.from('randomPhoneNumber')],
			program.programId
		);

		const global = await program.account.global.fetch(globalPda);

		await program.methods
			.createIdentity({
				phoneNumber: randomPhoneNumber,
			})
			.accounts({
				global: globalPda,
				identity: identityPda,
				owner: owner.publicKey,
				treasury: global.treasury,
				validator: validator.publicKey,
				systemProgram: SystemProgram.programId,
			})
			.signers([owner, validator])
			.rpc();

		const result = await program.account.identity.fetch(identityPda);

		assert.ok(owner.publicKey.equals(result.owner));
	});
});
