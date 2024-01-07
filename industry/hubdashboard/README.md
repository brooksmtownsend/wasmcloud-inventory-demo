# Hub Dashboard Microservice

This microservice, built in Rust, manages and displays a hub dashboard, handling inventory data and requests for inventory rundowns.

## Features

- **Handle Inventory Data**: Processes inventory updates from different branches and stores them using a key-value store.
- **Request Inventory Rundowns**: Facilitates users in requesting rundowns of inventory data.
- **Serve Dashboard UI**: Provides a user interface for viewing and interacting with the hub dashboard.

## Dependencies

This microservice utilizes the following wasmCloud capabilities:

- `wasmbus_rpc::actor::prelude::*` for Actor support.
- `wasmcloud_interface_httpserver` to serve the dashboard and handle HTTP requests.
- `wasmcloud_interface_keyvalue` for managing inventory data storage.
- `wasmcloud_interface_messaging` to handle messages and requests for rundowns.

## Endpoints

- `GET /rundown`: Initiates a request to publish a request rundown for inventory data.
- `GET /inventory`: Retrieves the inventory data for all branches.

## Data Structures

- `InventoryItem`: Represents an item in the inventory with attributes like branch, item type, and quantity.

## Message Handling

Handles incoming messages related to inventory updates. Validates and processes the data before storing it.

## Error Handling

Includes error handling for failed deserialization of messages and HTTP requests, providing appropriate responses.

## Usage

Deploy this microservice with wasmCloud to manage and display inventory data from all deployed units.

## Contributing

Contributors are welcome to improve functionality or add new features. Please adhere to Rust's coding standards and provide tests for new features.
