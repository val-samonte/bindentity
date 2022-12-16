import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import {
	Keypair,
	LAMPORTS_PER_SOL,
	sendAndConfirmTransaction,
	SystemProgram,
	Transaction,
} from '@solana/web3.js';
import { BN } from 'bn.js';
import { assert } from 'chai';
import { Bindentity } from '../target/types/bindentity';

describe('Provider Management', () => {
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.Bindentity as Program<Bindentity>;

	const [globalPda] = findProgramAddressSync(
		[Buffer.from('global')],
		program.programId
	);

	const providerOwner = Keypair.generate();
	const validatorSigner = Keypair.generate();
	const user = Keypair.generate();
	const bindentityName = 'sample_' + Math.floor(Math.random() * 100_000_000);

	const [providerPda] = findProgramAddressSync(
		[Buffer.from('provider'), Buffer.from(bindentityName, 'utf-8')],
		program.programId
	);

	const [validatorPda] = findProgramAddressSync(
		[
			Buffer.from('validator'),
			providerPda.toBytes(),
			validatorSigner.publicKey.toBytes(),
		],
		program.programId
	);

	before(async () => {
		const connection = program.provider.connection;

		// provider creation fee is 1 SOL, and we can only request 1 SOL airdrop at a time
		// airdrop 1
		try {
			const latestBlockHash = await connection.getLatestBlockhash();
			const signature = await connection.requestAirdrop(
				providerOwner.publicKey,
				LAMPORTS_PER_SOL
			);

			await connection.confirmTransaction({
				blockhash: latestBlockHash.blockhash,
				lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
				signature,
			});
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}

		// airdrop 2
		try {
			const latestBlockHash = await connection.getLatestBlockhash();
			const signature = await connection.requestAirdrop(
				providerOwner.publicKey,
				LAMPORTS_PER_SOL
			);

			await connection.confirmTransaction({
				blockhash: latestBlockHash.blockhash,
				lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
				signature,
			});
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}

		// send a few funds to the user who will avail the bindentity
		try {
			await sendAndConfirmTransaction(
				connection,
				new Transaction().add(
					SystemProgram.transfer({
						fromPubkey: providerOwner.publicKey,
						toPubkey: user.publicKey,
						lamports: LAMPORTS_PER_SOL / 10,
					})
				),
				[providerOwner]
			);
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('should create a new bindentity provider', async () => {
		const params = {
			name: bindentityName,
			published: false,
			registrationFee: new BN(0),
			uri: '',
			providerTreasury: providerOwner.publicKey,
		};

		const global = await program.account.global.fetch(globalPda);

		try {
			await program.methods
				.createProvider(params)
				.accounts({
					global: globalPda,
					owner: providerOwner.publicKey,
					provider: providerPda,
					treasury: global.treasury,
					systemProgram: SystemProgram.programId,
				})
				.signers([providerOwner])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}

		const provider = await program.account.provider.fetch(providerPda);

		assert.equal(provider.name, bindentityName);
	});

	it('should add a validator', async () => {
		try {
			await program.methods
				.createValidator({
					enabled: false,
					signer: validatorSigner.publicKey,
				})
				.accounts({
					authority: providerOwner.publicKey,
					provider: providerPda,
					validator: validatorPda,
					systemProgram: SystemProgram.programId,
				})
				.signers([providerOwner])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	xit('should not allow the user to register when using a disabled provider', async () => {});

	it('should update provider config', async () => {
		try {
			await program.methods
				.updateProvider({
					flags: 2,
					authority: null,
					registrationFee: null,
					treasury: null,
					uri: null,
				})
				.accounts({
					authority: providerOwner.publicKey,
					provider: providerPda,
				})
				.signers([providerOwner])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	xit('should not allow the user to register when using a disabled validator', async () => {});

	it('should enable a validator', async () => {
		const validator = await program.account.validator.fetch(validatorPda);

		try {
			await program.methods
				.updateValidator({
					flags: validator.flags | 7,
				})
				.accounts({
					authority: providerOwner.publicKey,
					provider: providerPda,
					validator: validatorPda,
				})
				.signers([providerOwner])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('should allow user to register to the new bindentity', async () => {
		const id = 'sample_user_id';
		const timestamp = new BN(Math.floor(new Date().getTime() / 1000));
		const [identityPda] = findProgramAddressSync(
			[
				Buffer.from('identity', 'utf-8'),
				Buffer.from(timestamp + '', 'utf-8'),
				providerPda.toBytes(),
				Buffer.from(id, 'utf-8'),
			],
			program.programId
		);

		const [linkPda] = findProgramAddressSync(
			[
				Buffer.from('link', 'utf-8'),
				providerPda.toBytes(),
				Buffer.from(id, 'utf-8'),
			],
			program.programId
		);

		const global = await program.account.global.fetch(globalPda);
		const provider = await program.account.provider.fetch(providerPda);

		try {
			await program.methods
				.createIdentity({
					id,
					registrationFee: new BN(0),
					timestamp,
				})
				.accounts({
					global: globalPda,
					identity: identityPda,
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
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});
});
