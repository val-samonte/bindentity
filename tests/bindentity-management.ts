import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from '@solana/web3.js';
import { Bindentity } from '../target/types/bindentity';
import { assert } from 'chai';
import validatorJSON from '../keys/validator.json';

describe('Bindentity Management', async () => {
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.Bindentity as Program<Bindentity>;
	const validatorKp = Keypair.fromSecretKey(new Uint8Array(validatorJSON));

	const [globalPda] = findProgramAddressSync(
		[Buffer.from('global')],
		program.programId
	);

	let global;
	let phoneProviderPda;
	let phoneProvider;
	let validatorPda;
	let validator;
	let owner: Keypair;
	let randomPhoneNumber;

	before(async () => {
		const _global = await program.account.global.fetchNullable(globalPda);

		if (!_global) {
			throw new Error(
				'Global config not found, please run `anchor run init` to initialize.'
			);
		}

		const [_phoneProviderPda] = findProgramAddressSync(
			[Buffer.from('provider'), Buffer.from('phone')],
			program.programId
		);

		const _phoneProvider = await program.account.provider.fetchNullable(
			_phoneProviderPda
		);

		if (!_phoneProvider) {
			throw new Error(
				'Phone Provider not found, please run `anchor run init` to initialize.'
			);
		}

		const validators = await program.account.validator.all([
			{
				memcmp: {
					offset: 10,
					bytes: _phoneProviderPda.toBase58(),
				},
			},
		]);

		if (validators.length === 0)
			throw new Error(
				'No validators for phone found, please run `anchor run init` to initialize.'
			);

		const { publicKey: _validatorPda, account: _validator } = validators[0];

		if (!_validator.signer.equals(validatorKp.publicKey)) {
			throw new Error('Validator Signer public key did not match');
		}

		global = _global;
		phoneProviderPda = _phoneProviderPda;
		phoneProvider = _phoneProvider;
		validatorPda = _validatorPda;
		validator = _validator;
		owner = Keypair.generate();
		randomPhoneNumber = Buffer.from(
			Math.floor(Math.random() * 100_000_000_000) + ''
		);

		try {
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
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('should register an identity', async () => {
		const timestamp = new anchor.BN(Math.floor(new Date().getTime() / 1000));
		const params = {
			id: randomPhoneNumber,
			timestamp,
			registrationFee: null,
		};

		const [identityPda] = findProgramAddressSync(
			[
				Buffer.from('identity'),
				Buffer.from(timestamp + ''),
				phoneProviderPda.toBytes(),
				randomPhoneNumber,
			],
			program.programId
		);

		const [linkPda] = findProgramAddressSync(
			[Buffer.from('link'), phoneProviderPda.toBytes(), randomPhoneNumber],
			program.programId
		);

		const accounts = {
			global: globalPda,
			identity: identityPda,
			link: linkPda,
			owner: owner.publicKey,
			provider: phoneProviderPda,
			providerTreasury: phoneProvider.treasury,
			signer: validator.signer,
			treasury: global.treasury,
			validator: validatorPda,
			systemProgram: SystemProgram.programId,
		};

		try {
			await program.methods
				.createIdentity(params)
				.accounts(accounts)
				.signers([owner, validatorKp])
				.rpc();

			const result = await program.account.identity.fetch(identityPda);
			assert.ok(owner.publicKey.equals(result.owner));
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('id owner should be able to void an identity', async () => {
		try {
			await program.methods
				.updateValidator({
					// allow validator to void
					flags: validator.flags | 4,
				})
				.accounts({
					authority: phoneProvider.authority,
					provider: phoneProviderPda,
					validator: validatorPda,
				})
				.rpc();
		} catch (e) {
			console.log(e);
		}

		const [linkPda] = findProgramAddressSync(
			[Buffer.from('link'), phoneProviderPda.toBytes(), randomPhoneNumber],
			program.programId
		);

		const link = await program.account.link.fetch(linkPda);

		try {
			await program.methods
				.voidIdentity({
					id: randomPhoneNumber,
				})
				.accounts({
					global: globalPda,
					identity: link.identity,
					link: linkPda,
					provider: phoneProviderPda,
					signer: program.provider.publicKey,
					treasury: phoneProvider.treasury,
					validator: validatorPda,
					validatorSigner: validator.signer,
					systemProgram: SystemProgram.programId,
				})
				.signers([validatorKp])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('should renew the same identity after getting void', async () => {
		const timestamp = new anchor.BN(Math.floor(new Date().getTime() / 1000));
		const params = {
			id: randomPhoneNumber,
			timestamp,
			registrationFee: null,
		};

		const [identityPda] = findProgramAddressSync(
			[
				Buffer.from('identity'),
				Buffer.from(timestamp + ''),
				phoneProviderPda.toBytes(),
				randomPhoneNumber,
			],
			program.programId
		);

		const [linkPda] = findProgramAddressSync(
			[Buffer.from('link'), phoneProviderPda.toBytes(), randomPhoneNumber],
			program.programId
		);

		const accounts = {
			global: globalPda,
			identity: identityPda,
			link: linkPda,
			owner: owner.publicKey,
			provider: phoneProviderPda,
			providerTreasury: phoneProvider.treasury,
			signer: validator.signer,
			treasury: global.treasury,
			validator: validatorPda,
			systemProgram: SystemProgram.programId,
		};

		try {
			await program.methods
				.createIdentity(params)
				.accounts(accounts)
				.signers([owner, validatorKp])
				.rpc();

			const result = await program.account.identity.fetch(identityPda);
			assert.ok(owner.publicKey.equals(result.owner));
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});

	it('owner should be able to void an identity using wallet', async () => {
		const [linkPda] = findProgramAddressSync(
			[Buffer.from('link'), phoneProviderPda.toBytes(), randomPhoneNumber],
			program.programId
		);

		const link = await program.account.link.fetch(linkPda);

		try {
			await program.methods
				.voidIdentity({
					id: null,
				})
				.accounts({
					global: globalPda,
					identity: link.identity,
					link: linkPda,
					provider: phoneProviderPda,
					signer: owner.publicKey,
					treasury: phoneProvider.treasury,
					validator: validatorPda,
					validatorSigner: validator.signer,
					systemProgram: SystemProgram.programId,
				})
				.signers([owner, validatorKp])
				.rpc();
		} catch (e) {
			console.log(e);
			throw new Error(e);
		}
	});
});
