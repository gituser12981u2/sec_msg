# Architectural Decision Record: Initial Architecture for Secure Messaging TUI

## Date

2024-07-25

## Status

Proposed

## Context

The purpose of this application is to be a secure, decentralized messaging application with a Terminal User Interface (TUI). The application needs to be highly portable, secure, and capable or running on a wide range of devices without centralized server.

## Decision

Here are the follow key architectural decisions:

1. Programming Language: Rust
    - Rationale: Rust offers memory safety, performance, and cross-platform compilation, allowing the application to run on a wide range of devices from "small, terminal only operating systems to windows or macOS"

2. Networking Library: libp2p
    - Rationale: libp2p provides a robust, decentralized networking stack that aligns with our goal of a serverless, peer-to-peer application.

3. Message Propagation Protocols: Floodsub and Gossipsub
    - Rationale: These protocols offer simple and efficient message propagation in a peer-to-peer network. They serve as a starting point, with plants to implement more protocols, including a security-focused one and a 1-on-1 protocol.

4. User Interface: Terminal User Interface (TUI)
    - Rationale: A TUI offers portability and lightweight operation, suitable for a wide range of devices.

5. Security Measures:
    - Elliptic Curve Diffie-Hellman (ECDH) key exchange
    - ECC encryption instead of AES-256
    - Checksums for message integrity
    - Perfect Forward Secrecy for each message
    - Ratoinale: These measures provide strong security while allowing user customization.

6. Network Features:
    - Comprehensive NAT traversal techniques
    - mDNS for local peer discovery
    - Distributed Hash Table (DHT) for peer discovery and routing
    - Decentralized Identifiers (DIDs) for identity management
    - Rationale: These features enhance connectivity and identity management in a decentralized network.

## Consequences

Positive:

- High portability across different devices and platforms
- Strong security features with user customization
- Decentralized architecture eliminating single points of failure
- Efficient peer-to-peer communication

Challenges:

- Implementing and maintaining multiple complex networking protocols
- Ensuring security measures are correctly implemented and do not introduce vulnerabilities
- Managing peer discovery and connectivity in various network environments and NAT configurations
- Balancing advanced features with a goal of simplicity and ease of use

## Future Considerations

- Develop of a custom, security-focused propagation protocol
- Implementation of a 1-on-1 messaging protocol
- Continuous evaluation and improvement of security measures
- Expansion of supported platforms and devices
