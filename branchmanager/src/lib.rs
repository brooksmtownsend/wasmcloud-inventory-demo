use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::*;

mod inventory;
use inventory::*;

// Data storage (KV)
// Store branch information at branch:info (need to find _some_ way to actually identify the
// branch, actors are stateless so it's gonna be hard to do this :dinkin:)
// Store paper at inventory:paper
// Store ink at inventory:ink
// Store printers at inventory:printer

// Handle new shipments on munderdifflin.shipments
// Handle new orders on munderdifflin.orders
// Listen on munderdifflin.rundown, publish all inventory contents to munderdifflin.rundown.<branch>

const TOPIC_PREFIX: &str = "munderdifflin.";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct BranchmanagerActor {}

#[async_trait]
impl MessageSubscriber for BranchmanagerActor {
    async fn handle_message(&self, _ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let topic = msg.subject.as_str().trim_start_matches(TOPIC_PREFIX);

        match topic {
            "shipments" => info!("Received shipment: {:?}", msg.body),
            "orders" => info!("Received order: {:?}", msg.body),
            "rundown" => info!("Received rundown request: {:?}", msg.body),
            _ => info!("Received unknown message: {:?}", msg.body),
        }

        Ok(())
    }
}
