use crate::{
	stats::PipelineMessage,
	server::req_resp::{ RequestHeader, ReqResp, TIMEOUT }
};
use std::{ net::TcpStream, error::Error, str::FromStr };
use timeout_io::Reader;


/// Handles a post request and updates the state
pub fn handle(header: RequestHeader, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	// Extract the POST data length
	let content_len = header.header_fields.get("Content-Length")
		.ok_or("No content length in POST request")?;
	let content_len = String::from_utf8(content_len.to_owned())?;
	let content_len = usize::from_str(&content_len)?;
	
	// Read POST body
	let mut post_data = vec![0u8; content_len];
	stream.read_exact(&mut post_data, TIMEOUT)?;
	let post_data = String::from_utf8(post_data)?;
	
	// Read message and check that it's not a UUID
	let message: PipelineMessage = serde_json::from_str(&post_data)?;
	message.update();
	
	// Write 200 OK
	stream.write_response_header("HTTP/1.1 200 Ok\r\n\r\n")
}