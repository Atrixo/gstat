use std::sync::RwLock;
use serde_derive::*;
use lazy_static::*;
use uuid::Uuid;


/// Declare a global state
lazy_static! {
	static ref GLOBAL_STATE: RwLock<PipelineMessage> = RwLock::new(PipelineMessage::default());
}


/// The current audio level
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Level {
	pub rms: f64,
	pub peak: f64,
	pub whatever: f64,
}


/// The GStreamer stats
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PipelineMessage {
	Uuid(Uuid),
	Error(String),
	Eos(String),
	Levels(Vec<Level>)
}
impl PipelineMessage {
	/// Puts `self` into the global state
	pub fn update(self) {
		match &self {
			PipelineMessage::Uuid(_) => (),
			_ => *GLOBAL_STATE.write().unwrap() = self
		}
	}
	/// The current `Stats`
	pub fn get() -> Self {
		GLOBAL_STATE.read().unwrap().clone()
	}
}
impl Default for PipelineMessage {
	fn default() -> Self {
		PipelineMessage::Error("No state available yet :(".to_string())
	}
}