use socketioxide::extract::{AckSender, Data, SocketRef};
use tracing::info;

pub fn handle_ping(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
    info!("Received event: {:?}", data);

    let event = match data.as_str() {
        "running" => Event::ClientRunning,
        "stop" => Event::ClientStop,
        "load" => Event::ClientLoad,
        _ => {
            error!("Unknown event received: {:?}", data);
            return;
        }
    };
    let state_machine = StateMachine { socket };
    ack.send("pong").ok();
    socket.emit("pong", "pong").ok();

}
