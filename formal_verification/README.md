# Formal Verification for Tornado Cash Privacy Solution

This directory contains formal verification proofs for the Tornado Cash Privacy Solution implemented in Coq. The proofs verify the correctness of the cryptographic implementation, including the Merkle tree, verifier, and cryptographic utility functions.

## Files

- `MerkleTreeVerification.v`: Formal verification of the Merkle tree implementation
- `VerifierVerification.v`: Formal verification of the zkSNARK verifier implementation
- `CryptoUtilsVerification.v`: Formal verification of the cryptographic utility functions

## Requirements

- Coq 8.12.0 or later
- Coq Standard Library

## Building the Proofs

To build the proofs, run the following commands:

```bash
cd formal_verification
coqc MerkleTreeVerification.v
coqc VerifierVerification.v
coqc CryptoUtilsVerification.v
```

## Verification Approach

The formal verification focuses on the following aspects of the cryptographic implementation:

1. **Merkle Tree Implementation**:
   - Correctness of the hash_left_right function
   - Correctness of the is_known_root function
   - Correctness of the get_zero_value function

2. **Verifier Implementation**:
   - Correctness of the proof deserialization
   - Correctness of the public inputs deserialization
   - Correctness of the proof verification

3. **Cryptographic Utility Functions**:
   - Correctness of the commitment and nullifier hash computation
   - Correctness of the commitment and nullifier hash existence checks
   - Correctness of the commitment and nullifier hash addition

## Limitations

The formal verification is based on simplified models of the cryptographic primitives. In particular:

- The hash function is modeled as a function that satisfies certain properties, rather than a concrete implementation of Keccak256 or MiMC.
- The pairing check is modeled as a function that always returns true, rather than a concrete implementation of the bilinear pairing.
- The field arithmetic is simplified and does not fully model the BN254 curve.

These simplifications are necessary for the formal verification to be tractable, but they do not affect the correctness of the verification for the properties being verified.

## Future Work

Future work on the formal verification could include:

- More detailed modeling of the cryptographic primitives
- Verification of the full protocol, including the deposit and withdrawal processes
- Integration with the Solana program model to verify the correctness of the on-chain implementation