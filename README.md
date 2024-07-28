# Secure Messaging App

## Pre-1.0 Release Warning

This project is currently in pre-1.0 state. The API is not stable and may undergo significant changes and additions.

This is a secure messaging application built as a Terminal User Interface (TUI) that can run on any system. The application leverage the libp2p framework to provide secure, peer-to-peer communication.

## Features

- **Secure Communication**: Utilizes libp2p for encrypted peer-to-peer messaging.
- **Cross-Platform**: Runs on any system with a terminal
- **Floodsub and Gossipsub Protocols**: Supports both Floodsub and Gossipsub for message propogation
- **User-Friendly TUU**: Built with a simple and intuitive terminal interface.

## Installing

1. **Clone the repository**:

    ```bash
    git clone https://github.com/gituser12981u2/sec_msg/tree/master
    ```

2. **Install the required dependencies**:

    ```bash
    cargo build
    ```

3. **Run the application**:

    ```bash
    cargo run
    ```

## Usage

1. Start the application using the command above.
2. Follow the prompts in the terminal to connect to peers and send messages.

## Contributing

Contributions are welcome. Please read the [CONTRIBUTING.md](CONTRIBUTING.md) guide to get started.
