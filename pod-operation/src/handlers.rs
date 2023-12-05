use socketioxide::extract::{AckSender, Data, SocketRef};
use tracing::info;

pub fn handle_ping(socket: SocketRef, Data(data): Data<String>, ack: AckSender) {
	info!("Received event: {:?}", data);
	ack.send("pong").ok();
	socket.emit("pong", "pong").ok();
}
