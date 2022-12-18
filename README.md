# Bindentity

Bindie (short for Bindentity, phonetically similar to ID) allows users to link their Solana wallet address to any data such as phone number, email, Discord ID, Twitter handle, Ethereum address, government IDs, school IDs and so on, while protecting the users' privacy.

## How does it work?

![Bindentity Overview](https://user-images.githubusercontent.com/767060/208278854-fc3f72a9-7202-47d7-9e2e-eacd61c4e36c.png)

The protocol allows the providers to distribute their own bindie under their own namespace. A namespace is a unique identifier of the provider. Some of these namespaces are already reserved by the protocol, such as `email` and `phone`.

Anyone can apply as a provider for 1 SOL (subject to change). The provider must supply the basic details such as name, description, logo, default registration fee, and purpose of the bindie.

Once created, the provider is required to create at least 1 validator so that the users can now finally avail the bindie. A validator's role is to verify and approve the user's application of the bindie. It can be a person, a backend app or a multisig, which will co-sign the user's bindie creation. It is up to the provider to implement the process of validation, thus the credibility of the bindie entirely depends on the provider.

As an example, the bindie providers `email` and `phone` (both being managed by the protocol) utilizes Google's Authentication service (Firebase). When the user tries to apply, he / she is required to authenticate using his / her email or phone number. The backend, which contains the validator secret key, validates the user's authentication, then approves and co-signs the user's request for registration. The program then stores the hash of the email or phone number on-chain, together with the user's Solana wallet address.

Since it is now stored on-chain, anyone can use this newly created bindie as a proof that the user's wallet address is indeed tied to the user's email or phone number. 

Note of course, a bindie provider is not only limited to store emails and phone numbers. Any data, such as company's employee ID, can be tied up to the user's wallet address.
