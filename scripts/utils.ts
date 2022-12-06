import { Keypair, PublicKey, Transaction } from '@solana/web3.js';

export interface AnchorWallet {
	publicKey: PublicKey;
	signTransaction(transaction: Transaction): Promise<Transaction>;
	signAllTransactions(transactions: Transaction[]): Promise<Transaction[]>;
}

export class KeypairWallet implements AnchorWallet {
	constructor(readonly payer: Keypair) {}

	async signTransaction(tx: Transaction): Promise<Transaction> {
		tx.partialSign(this.payer);
		return tx;
	}

	async signAllTransactions(txs: Transaction[]): Promise<Transaction[]> {
		return txs.map((tx) => {
			tx.partialSign(this.payer);
			return tx;
		});
	}

	get publicKey(): PublicKey {
		return this.payer.publicKey;
	}
}
