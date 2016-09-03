use std::vec::Vec;

// Nothing really useful here, just stubs for when I come back to this
pub trait Protocol<T> {
	fn decode(&self, bytes: &[u8]) -> Result<T, &'static str>;
	fn encode(&self, message: &T) -> Vec<u8>;
}
