use crate::{
	stats::PipelineMessage,
	server::req_resp::{ RequestHeader, ReqResp, TIMEOUT }
};
use std::{ net::TcpStream, error::Error };
use timeout_io::Writer;


/// The site template
const SITE: &'static str = include_str!("site.html");


/// Handles a get request
pub fn handle(header: RequestHeader, stream: TcpStream) -> Result<(), Box<dyn Error>> {
	// Check if the state was requested
	match header.path.as_str() {
		"/state.json" => deliver_state(stream),
		_ => deliver_html(stream)
	}
}


/// Delivers the current state as JSON
fn deliver_state(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	// Encode state
	let state_json = serde_json::to_string(&PipelineMessage::get())?;
	
	// Write state
	stream.write_response_header(&format!(
		concat!(
			"HTTP/1.1 200 Ok\r\n",
			"Content-Type: text/html\r\n",
			"Content-Length: {}\r\n\r\n"
		), state_json.len()
	))?;
	Ok(stream.write_exact(state_json.as_bytes(), TIMEOUT)?)
}


/// Delivers the HTML page
fn deliver_html(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	// Write HTML
	stream.write_response_header(&format!(
		concat!(
			"HTTP/1.1 200 Ok\r\n",
			"Content-Type: text/html\r\n",
			"Content-Length: {}\r\n\r\n"
		), SITE.len()
	))?;
	Ok(stream.write_exact(SITE.as_bytes(), TIMEOUT)?)
}
