use std::{ time::Duration, collections::HashMap, error::Error };
use httparse::{ EMPTY_HEADER, Request };
use timeout_io::{ Reader, Writer };


/// The default timeout for TCP-IO-operations
pub const TIMEOUT: Duration = Duration::from_secs(30);


/// A request header
pub struct RequestHeader {
	/// The HTTP method
	pub method: String,
	/// The requested resource
	pub path: String,
	/// The HTTP version
	pub version: u8,
	/// The header fields
	pub header_fields: HashMap<String, Vec<u8>>
}
impl RequestHeader {
	/// Parses the header from `bytes`
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
		// Allocate vars
		let mut header_lines = [EMPTY_HEADER; 64];
		let mut request = Request::new(&mut header_lines);
		
		// Parse header
		let header_lines_len = match request.parse(&bytes) {
			Ok(r) if r.is_complete() => r.unwrap(),
			Ok(_) => Err("Truncated HTTP header")?,
			Err(_) => Err("Invalid HTTP request header")?
		};
		
		// Build a hash map over the header lines
		let mut header_fields = HashMap::new();
		request.headers.iter().take(header_lines_len).for_each(|h| {
			header_fields.insert(h.name.to_string(), h.value.to_vec());
		});
		
		// Extract elements
		Ok(Self {
			method: request.method.ok_or("Invalid HTTP header (missing method)")?.to_string(),
			path: request.path.ok_or("Invalid HTTP header (missing path)")?.to_string(),
			version: request.version.ok_or("Invalid HTTP header (missing version)")?,
			header_fields
		})
	}
}


pub trait ReqResp {
	/// Reads the request header
	fn read_request_header(&mut self) -> Result<RequestHeader, Box<dyn Error>>;
	/// Writes the response header
	fn write_response_header(&mut self, header: &str) -> Result<(), Box<dyn Error>>;
}
impl<T: Reader + Writer> ReqResp for T {
	fn read_request_header(&mut self) -> Result<RequestHeader, Box<dyn Error>> {
		// Read HTTP header
		let mut header_data = vec![0u8; 4096];
		let len = self
			.read_until(&mut header_data, b"\r\n\r\n", TIMEOUT)
			.map_err(|e| format!("Failed to read HTTP header: {}", e))?
			.ok_or(format!("Maximum header length ({}) exceeded", header_data.len()))?;
		
		// Parse header
		header_data.truncate(len);
		Ok(RequestHeader::from_bytes(&header_data)?)
	}
	fn write_response_header(&mut self, header: &str) -> Result<(), Box<dyn Error>> {
		Ok(self.write_exact(header.as_bytes(), TIMEOUT)?)
	}
}