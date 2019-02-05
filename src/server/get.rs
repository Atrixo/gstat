use crate::{
	stats::PipelineMessage,
	server::req_resp::{ RequestHeader, ReqResp, TIMEOUT }
};
use std::{ net::TcpStream, error::Error };
use timeout_io::Writer;


/// The site template
const SITE_TEMPLATE: &'static str = include_str!("site.template.html");


/// Handles a get request
pub fn handle(_header: RequestHeader, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	// Build HTML site
	let state = PipelineMessage::get();
	let html = SITE_TEMPLATE.replace("$PLACEHOLDER$", &format!("{:#?}", state));
	
	// Write response header
	stream.write_response_header(&format!(
		concat!(
			"HTTP/1.1 200 Ok\r\n",
			"Content-Type: text/html\r\n",
			"Content-Length: {}\r\n\r\n"
		), html.len()
	))?;
	Ok(stream.write_exact(html.as_bytes(), TIMEOUT)?)
}