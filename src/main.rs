mod server;
mod stats;

use crate::server::Server;

fn main() {
	// Spawn the server
	Server::start_runloop("localhost:8080");
}
