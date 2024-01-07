# Unit Manager Microservice

This microservice, developed in Rust, is designed to manage and display inventory data for individual units. It handles inventory updates, processes shipments and orders, and serves a dashboard UI for unit management.

## Features

- **Handle Inventory Data**: Processes inventory updates and manages inventory data for each unit.
- **Process Shipments and Orders**: Handles incoming shipments and outgoing orders, updating inventory accordingly.
- **Serve Dashboard UI**: Provides a user interface for viewing and interacting with individual unit dashboards.
- **Manage Unit Information**: Allows setting and retrieving the name of the unit.

## Dependencies

This microservice utilizes the following wasmCloud capabilities:

- `wasmbus_rpc::actor::prelude::*` for Actor support.
- `wasmcloud_interface_httpserver` to serve the dashboard and handle HTTP requests.
- `wasmcloud_interface_keyvalue` for managing inventory data storage.
- `wasmcloud_interface_logging` for logging information.
- `wasmcloud_interface_messaging` to handle messages related to inventory rundowns.

## Endpoints

- `GET /inventory`: Retrieves the current inventory data for the unit.
- `POST /shipment`: Processes a new shipment, adding items to the inventory.
- `POST /order`: Processes a new order, removing items from the inventory.
- `GET /name`: Retrieves the name of the unit.
- `POST /name`: Sets the name of the unit.

## Data Structures

- `InventoryItem`: Represents an item in the inventory with attributes such as item type and quantity.

## Message Handling

Handles incoming messages related to inventory rundowns and updates. Processes data before storing and facilitates communication between units.

## Error Handling

Implements error handling for deserialization issues, HTTP request failures, and insufficient inventory for orders, providing appropriate responses.

## Usage

Deploy this microservice within a wasmCloud host to manage and display inventory data for individual units.

## Contributing

Contributors are encouraged to enhance the functionality or add new features. Please follow Rust's coding standards and include tests for new implementations.
