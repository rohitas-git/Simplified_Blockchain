# Database

Substrate makes no assumptions about the content or structure of the data in your blockchain. The underlying database layer uses simple key-value storage, on top of which a modified Patricia Merkle tree (trie) is implemented. This special storage structure allows us to easily verify if an item is or is not in that storage. This is particularly important to support light clients, who will depend on these storage proofs to provide light-weight, yet trustless interactions with the blockchain network.

# Networking

Substrate uses libp2p as a modular peer-to-peer networking stack. Through this networking layer, Substrate-based blockchains are able to share transactions, blocks, peers, and other system critical details without the need for centralized servers. In line with Substrate’s philosophy, libp2p is unique in the fact that it makes no assumptions about your specific networking protocol. As a result, you are able to implement and use different transports on top of a Substrate-based blockchain.

# Transaction Queue

As mentioned above, transactions are collected and formed into blocks that ultimately define how the state of the blockchain changes. However, the order of these transactions can impact the final state of the ledger. Substrate allows you full control over the dependency and queue management of transactions on your network. Substrate only assumes that a transaction has a weight and a set of prerequisite tags that are used to create dependency graphs. These dependency graphs are linear in the simplest case, but they can become more complex. Substrate handles those complexities for you automatically.

# Consensus
Recall that there are different ways that a blockchain network can come to consensus about changes to the chain. Traditionally, these consensus engines are tightly coupled to the other blockchain components. However, Substrate has spent extra effort designing a consensus layer that can be easily changed during development. In fact, it was made such that consensus could even be hot-swapped after the chain goes live! Built into Substrate are multiple different consensus engines such as traditional Proof of Work (PoW), Aura (Authority Round), and Polkadot consensus, which is unique in the fact that it separates the block production process (BABE) from the block finalization process (GRANDPA).