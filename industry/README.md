# Industry Inventory Demo

The industry inventory is a sample application that demonstrates how wasmCloud actors can be used to build a distributed application. The application is a simple inventory management system that lets a central server track the inventory of a number of warehouses or hubs. The inventory items tracked are generic and can meet the needs of any industry. The application is built using the following components:

1. [Unit Manager](./unitmanager/): a wasmCloud actor that provides a web-based dashboard and RESTful API for managing the inventory of a single warehouse or unit. This is designed to be deployed on a Raspberry Pi or other small device that can be placed in a warehouse or unit.
1. [Hub Dashboard](./hubdashboard): a wasmCloud actor that provides a web-based dashboard for a hub. The dashboard allows the user to view the inventory of every and to add or remove items from the inventory. This is designed to be deployed in a central location like a cloud and to be used by a hub manager.

## Architecture

![Architecture](./architecture.png)

## Deployment

You can deploy this sample application on Cosmonic using the included [wadm.yaml](./wadm.yaml) manifest.

```
cosmo up
cosmo app deploy ./wadm.yaml
```

## KubeCon / WasmCon

At both KubeCon and WasmCon, this demonstration was used to display a particular inventory for a paper company. Since then, the demonstration has been updated to be more generic and to allow for more flexibility in the inventory items.
