# Luban the Paymaster - A Cross-chain Tx Sponsorship Protocol
![Capture](https://github.com/da-bao-jian/eth-waterloo/assets/63417973/4684c09e-cdf3-40c9-bc12-da93d950b963)

Let's paint a picture. You're a high-rolling Ethereum whale, frequenting the dark corners of Discord channels, always on the hunt for the next big alpha. Suddenly, a hot tip comes in - there's a token launch happening on Polygon, and you, being the adventurous degen you are, want to dive right in. But alas! Your treasure chest is all on Ethereum. You're a fish out of water on Polygon.

You've got 10 minutes on the clock. The pressure is on. You're sweating bullets. 😰

Now, if you know what you're doing, you would:

1. Scour the seven seas for the fastest bridge,
2. Navigate to the bridge's website,
3. Connect your Ethereum wallet,
4. Initiate the bridge transaction,
5. Wait... and wait... and wait...
6. Finally, arrive at Polygon and connect your wallet,
7. Only to find out that you've missed the boat. The token launch is over.
8. Sounds like a nightmare, doesn't it?

But what if I told you, you could skip all these steps? What if you could teleport from Ethereum to Polygon in a single transaction? What if you could do it in the time it takes to make a regular transaction? And what if you didn't even need a wallet on Polygon?

Luban the Paymaster, your cross-chain fairy godfather. With a wave of his magic wand, he helps you get from chain A to chain B within seconds. No bridges, no waiting, no hassle.

# How Luban works? 
![Capture](https://github.com/da-bao-jian/eth-waterloo/assets/63417973/232b809f-43a7-45e9-a870-76c19e03b5b1)

Luban operates by utilizing a custom paymaster - `CrosschainPaymaster.sol` - designed for Account Abstraction wallets, in conjunction with a unique JSON-RPC method, eth_checkCrossChainBalance. Here's a step-by-step breakdown of how it works:

* User Balance on Hosting Chain: The user needs to maintain a balance only on the hosting chain. This is achieved by depositing assets into the AccountEscrow.sol singleton contract on that chain. The deposit can be any ERC20 token or ETH.
  
* User Transaction on Transacting Chain: When a user wants to transact on a different chain, they submit a UserOperation with the `paymasterAndData` field pointing to the `CrosschainPaymaster.sol` address on the target chain, along with the chain ID of the hosting chain
  
* Balance Check: The bundler initiates a JSON-RPC `eth_checkCrossChainBalance` call to verify if the user has sufficient balance in the `AccountEscrow.sol` on the hosting chain. If the balance is adequate, the CrosschainPaymaster.sol sponsors the transaction. If not, the UserOperation is discarded.

* Transaction Flow: The UserOperation then follows the regular transaction flow. The key difference is that during the `postOp` phase, CrosschainPaymaster.sol initiates a cross-chain transfer call to deduct the user's deposit from the `AccountEscrow.sol`.

* Universal Application: This pattern is applicable to any transactions, swap, smart contract wallet deployment, adjust liquidity positions on AMMs.

In the future, it's conceivable that a user would only need to maintain assets on a single EVM chain. These assets could then be used to sponsor transactions on all other EVM chains, as long as Luban is there to facilitate the process. This should be the pattern for future cross-chain transactions. 
  
