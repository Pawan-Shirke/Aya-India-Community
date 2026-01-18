🆔 Identity Lifecycle Vault

A Solana smart contract that enforces identity lifecycle continuity by recording and governing identity update events as on-chain state. Built with Rust and Anchor to demonstrate how blockchain can support auditable, self-sovereign digital identity systems.

🎯 Project Summary

The Identity Lifecycle Vault treats digital identity as a time-evolving on-chain state rather than a one-time registration. Each identity is represented by a program-controlled account that records enrolment and update events. On-chain timestamps enforce lifecycle discipline, preventing neglected or unauditable identity states.

⚙️ How It Works

Initialize: The user calls initialize_identity, creating a Program Derived Address (PDA) that represents their identity vault and records the creation timestamp.

Update: The user records lifecycle events (biometric or demographic updates). Each update mutates the vault state by incrementing counters, updating timestamps, and adjusting an engagement score.

Validate: All updates use Solana’s on-chain Clock sysvar to ensure time-ordered, auditable lifecycle transitions.

Close: The user may optionally close the identity vault, at which point the account is closed and all rent is refunded to the owner.

🛠️ Tech Stack

Language: Rust
Framework: Anchor
Network: Solana Devnet

🔒 Security Features

Signer Checks: Only the identity owner can mutate or close the vault.
Time-Aware Logic: Lifecycle events are recorded using the on-chain Clock sysvar.
PDA State: Identity data is stored in program-controlled accounts, not personal wallets.
