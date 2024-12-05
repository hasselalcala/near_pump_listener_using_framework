# NEAR Event Listener Client

A robust event listener implementation for monitoring and processing events on the NEAR Protocol blockchain, with specific focus on token-related events.

## Overview

This project implements a specialized event listener for the NEAR blockchain that monitors token-related events, processes them, and stores them in a PostgreSQL database. It's designed with a focus on reliability, efficient event processing, and proper database management.

## Features

* **Event Monitoring**:
  * Real-time event listening
  * Configurable account and method filtering
  * Automatic block height tracking
  * Event batching and processing

* **Database Integration**:
  * PostgreSQL support
  * Connection pooling with bb8
  * Automatic schema creation
  * Efficient token storage and retrieval

* **Network Support**:
  * Testnet and Mainnet compatibility
  * Configurable RPC endpoints
  * Environment-specific settings

* **Token Management**:
  * Complete token metadata storage
  * Support for NFT standards
  * Structured token data model

## Prerequisites

* Rust (latest stable version)
* PostgreSQL database
* NEAR account (Testnet or Mainnet)
* Environment variables configuration

## Installation

1. Clone the repository:

```
git clone https://github.com/hasselalcala/near_pump_listener_using_framework.git
cd near_pump_listener_using_framework
```

2. Build the project:

```
cargo build
```


## Usage

The client can be run with different network configurations using command-line arguments:

```
cargo run -- --network [testnet|mainnet]
```


## Configuration

Key configuration constants are stored in `src/constants.rs`:

* `ACCOUNT_TO_LISTEN`: The account ID to listen for events.
* `FUNCTION_TO_LISTEN`: The function name to listen for events.
* `NEAR_RPC_MAINNET`: The RPC URL for the Mainnet.
* `NEAR_RPC_TESTNET`: The RPC URL for the Testnet.


## Database Schema

The project uses two main tables:

1. `block_height`: Tracks the last processed block
2. `tokens`: Stores token information with the following structure:
   - owner_id
   - total_supply
   - spec
   - name
   - symbol
   - icon
   - reference
   - reference_hash
   - decimals
   - image
   - description
   - auction_duration
   - min_buy_amount

## Development

### Running Tests

```
cargo test
```

## Architecture

The project follows a modular architecture with several key components:

* **CLI Module**: Handles command-line arguments and configuration
* **Constants**: Stores configuration constants
* **Database**: Manages database connections and operations
* **Models**: Defines data structures and DTOs
* **Event Listener**: Core functionality for monitoring blockchain events


## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

* NEAR Protocol team
* near-sdk-rs developers
* PostgreSQL community

