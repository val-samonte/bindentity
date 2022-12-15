// import { AnchorProvider, BN, Program } from '@project-serum/anchor';
// import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
// import {
// 	Keypair,
// 	LAMPORTS_PER_SOL,
// 	PublicKey,
// 	SystemProgram,
// } from '@solana/web3.js';

// import idl from '../target/idl/mobile_number_identity.json';
// import { MobileNumberIdentity } from '../target/types/mobile_number_identity';

// import programKp from '../target/deploy/mobile_number_identity-keypair.json';
// import authorityKp from '../keys/authority.json';
// import validatorKp from '../keys/validator.json';
// import treasuryKp from '../keys/treasury.json';
// import { KeypairWallet } from './utils';

// const { publicKey: programId } = Keypair.fromSecretKey(
// 	new Uint8Array(programKp)
// );
// const authority = Keypair.fromSecretKey(new Uint8Array(authorityKp));
// const validator = Keypair.fromSecretKey(new Uint8Array(validatorKp));
// const treasury = Keypair.fromSecretKey(new Uint8Array(treasuryKp));

// const program = new Program<MobileNumberIdentity>(
// 	idl as unknown as MobileNumberIdentity,
// 	programId,
// 	new AnchorProvider(
// 		AnchorProvider.env().connection,
// 		new KeypairWallet(authority),
// 		{}
// 	)
// );

// const [programDataPda] = findProgramAddressSync(
// 	[programId.toBytes()],
// 	new PublicKey('BPFLoaderUpgradeab1e11111111111111111111111')
// );

// const [globalPda] = findProgramAddressSync([Buffer.from('global')], programId);

// const init = async () => {
// 	try {
// 		await program.methods
// 			.initialize({
// 				creationFee: new BN(LAMPORTS_PER_SOL * 0.005),
// 				treasury: treasury.publicKey,
// 				validator: validator.publicKey,
// 			})
// 			.accounts({
// 				global: globalPda,
// 				authority: authority.publicKey,
// 				program: programId,
// 				programData: programDataPda,
// 				systemProgram: SystemProgram.programId,
// 			})
// 			.rpc();

// 		const global = await program.account.global.fetch(globalPda);
// 		console.log('Global config initialized:', JSON.stringify(global, null, 2));
// 	} catch (e) {
// 		try {
// 			const global = await program.account.global.fetch(globalPda);
// 			console.log(
// 				'Global config already exist:',
// 				JSON.stringify(global, null, 2)
// 			);
// 		} catch (e) {
// 			console.log(e);
// 		}
// 	}
// };

// init();
