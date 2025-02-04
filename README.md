# Shamir's Secret Sharing with Verifiable Secret Sharing (VSS)

## Overview
`shamir_vss` is a Rust-based implementation of Shamir's Secret Sharing (SSS) algorithm with Verifiable Secret Sharing (VSS). The primary goal of this project is to securely split a secret into multiple shares, ensuring that only a predefined threshold of shares can reconstruct the original secret while providing cryptographic proof of share validity.

## Key Features
- **Shamir's Secret Sharing (SSS)**: Securely splits a secret into multiple shares.
- **Threshold-Based Reconstruction**: Requires a minimum `k` shares out of `n` to recover the secret.
- **Verifiable Secret Sharing (VSS)**: Uses cryptographic commitments to ensure the integrity of shares.
- **Multi-Base Support**: Handles values in various numeral bases (binary, octal, decimal, hexadecimal, etc.).
- **Modular Arithmetic & GCD Optimization**: Efficient rational number arithmetic and greatest common divisor calculations.
- **Commitment Generation with Modular Exponentiation**: Uses a base generator `G` and prime modulus `P` to validate shares.
- **Error Handling**: Detects and prevents invalid or malicious shares from being used in reconstruction.

## How It Works
- **Share Parsing & Base Conversion**: Converts values from different bases into decimal for computation.
- **Secret Sharing**: The secret is divided into `n` shares, requiring `k` shares for reconstruction.
- **Commitment Generation**: Uses modular exponentiation `G^coeff mod P` to validate shares.
- **Secret Recovery**: Given at least `k` valid shares, the original secret is reconstructed using Lagrange interpolation.
- **Modular Arithmetic Operations**: Efficient rational number multiplication, addition, and reduction to simplify computations.

## Future Enhancements
- Implement elliptic curve-based secret sharing for enhanced security.

