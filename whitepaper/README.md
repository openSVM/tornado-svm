# Tornado Cash Privacy Solution for Solana - Whitepaper

This directory contains the academic whitepaper for the Tornado Cash Privacy Solution for Solana, written in ArXiv style and formatted using Typst.

## Files

- `tornado-svm-whitepaper.typ` - The main Typst source file for the whitepaper
- `references.bib` - Bibliography file containing all references
- `tornado-svm-whitepaper.pdf` - The compiled PDF version of the whitepaper

## Abstract

This whitepaper presents Tornado Cash Privacy Solution for Solana, a non-custodial privacy protocol that enables private transactions on the Solana blockchain using zero-knowledge succinct non-interactive arguments of knowledge (zkSNARKs). The protocol breaks the on-chain link between sender and recipient addresses through a commitment-nullifier scheme backed by Merkle tree proofs.

## Key Contributions

1. **Solana Optimization**: Adaptation of Tornado Cash protocol for Solana's execution environment
2. **Formal Verification**: Coq proofs for correctness of core cryptographic components
3. **Security Analysis**: Comprehensive threat modeling and attack analysis
4. **Performance Evaluation**: Detailed analysis of gas costs and scalability characteristics
5. **Implementation Details**: Complete open-source implementation with client libraries

## Compilation

To compile the whitepaper from source:

```bash
# Install Typst
curl -fsSL https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz | tar -xJ
sudo mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/

# Compile the whitepaper
typst compile tornado-svm-whitepaper.typ
```

## Contents

The whitepaper covers:

- **Introduction and Background**: Motivation for privacy in blockchain transactions
- **Cryptographic Foundations**: zkSNARKs, Merkle trees, and commitment schemes
- **Protocol Design**: Detailed description of the deposit and withdrawal process
- **Formal Verification**: Mathematical proofs of correctness implemented in Coq
- **Security Analysis**: Threat model and attack resistance analysis
- **Implementation Details**: Solana-specific optimizations and architecture
- **Performance Analysis**: Gas costs, proof generation times, and scalability
- **Audit and Compliance**: Security findings and regulatory considerations
- **Future Work**: Planned improvements and research directions

## Formal Verification

The whitepaper includes detailed analysis of the formal verification proofs implemented in Coq, covering:

- Merkle tree correctness properties
- Commitment scheme security (hiding and binding)
- Nullifier uniqueness and double-spending prevention
- Implementation correctness for critical functions

## Author

Rin Fhenzig @ OpenSVM

## Citation

If you use this work in your research, please cite:

```bibtex
@article{fhenzig2024tornado,
  title={Tornado Cash Privacy Solution for Solana: A zkSNARK-based Private Transaction Protocol},
  author={Fhenzig, Rin},
  journal={OpenSVM Research},
  year={2024},
  organization={OpenSVM}
}
```

## License

This whitepaper is released under the same MIT license as the accompanying source code.
