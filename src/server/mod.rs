mod req_resp;
mod get;
mod post;


use self::req_resp::ReqResp;
use std::{ thread, net::{ ToSocketAddrs, TcpListener, TcpStream } };


/// Returns either the result of `$e` or prints the error and returns
macro_rules! ok_or_die {
	($e:expr) => (match $e {
		Ok(r) => r,
		Err(e) => {
			eprintln!("Non fatal error on connection:\n\t{}", e);
			return
		}
	});
}


/// A HTTP server
pub struct Server;
impl Server {
	/// The server's runloop
	pub fn start_runloop(addr: impl ToSocketAddrs) -> ! {
		let socket = TcpListener::bind(addr).unwrap();
		loop {
			let stream = match socket.accept() {
				Ok((stream, _)) => stream,
				Err(e) => {
					eprintln!("Failed to accept connection: {}", e);
					continue
				}
			};
			thread::spawn(move || Self::handle(stream));
		}
	}
	
	/// Handle a HTTP connection
	fn handle(mut stream: TcpStream) {
		// Read header
		let header = ok_or_die!(stream.read_request_header());
		
		// Validate HTTP version
		if header.version != 1 {
			eprintln!("Invalid HTTP version in request (v{})", &header.version);
			return
		}
		
		// Handle request
		ok_or_die!(match header.method.as_str() {
			"GET" => get::handle(header, stream),
			"POST" => post::handle(header, stream),
			_ => stream.write_response_header(concat!(
				"HTTP/1.1 405 Method Not Allowed\r\n",
				"Allow: GET,POST\r\n\r\n"
			))
		})
	}
}