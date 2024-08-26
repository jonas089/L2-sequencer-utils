:warning: This repository describes an experimental consensus protcol that has not yet been used in any network. Proceed with caution! :warning:

# PoRD-SQ: Decentralized Sequencer
See [here](https://github.com/jonas089/PoRD-sequencer/blob/master/src%2Fconsensus%2Flogic.rs) an implementation of PoRD.

Review: ZK might not be necessary here as a VRF could suffice. However it does work with ZK random numbers so for now this writeup is not being updated.

# Proof of Random Delta

PoRD is a consensus protocol where the odds of each validator being selected to create the next block are always equal.

Consensus is established over a sequence of commitments (zero knowledge random numbers).

A 51%+ majority of the known validators must commit to a block for it to be valid.

For more information, read `./whitepaper`.