# Branch Manager Actor Microservice

## Overview

The Branch Manager Actor is a wasmCloud microservice developed in Rust to manage inventory for a paper company inspired by the renowned "Dunder Mifflin" from the television series "The Office". This microservice responds to various topics and performs different inventory operations like handling new shipments, orders, and inventory rundowns.

This actor utilizes several wasmCloud interfaces such as `wasmbus_rpc`, `wasmcloud_interface_keyvalue`, `wasmcloud_interface_logging`, `wasmcloud_interface_httpserver` and `wasmcloud_interface_messaging` to facilitate interaction with other components and services in the wasmCloud ecosystem.

## Features

- **Inventory Management**: This actor keeps track of inventory items and updates quantities based on shipments and orders.
- **Message Subscription**: It listens to different topics and responds accordingly to manage inventory updates and rundowns.
- **Logging**: Integrated logging feature to log unknown messages.

## Message Topics

The actor responds to the following message topics:

- **munderdifflin.rundown**: To publish a rundown of all inventory contents.

## HTTP Endpoints

The actor supports the following `POST` endpoints to update inventory:

- **/shipment**: To handle new shipments and update the inventory accordingly.
- **/order**: To process new orders and update the inventory.

## Dependencies

This actor uses the following wasmCloud interfaces:

- `wasmbus_rpc`: To define actor and message subscriber.
- `wasmcloud_interface_keyvalue`: To perform key-value operations for inventory management.
- `wasmcloud_interface_logging`: For logging capabilities.
- `wasmcloud_interface_messaging`: To handle messaging functionalities.
- `wasmcloud_interface_httpserver`: To handle local HTTP requests for inventory management.

## Usage

This actor is meant to be a run as a part of a broader [industry](../) demo application. Refer to the README in that parent directory for instructions on how to run the demo.
