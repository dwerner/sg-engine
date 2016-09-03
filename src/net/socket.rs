pub struct SocketWrapper {
	blah: u8
}

impl SocketWrapper {

	pub fn new(blah: u8) -> Self {
		SocketWrapper{ blah: blah }
	}

	pub fn get_blah(&self) -> u8 {
		self.blah
	}

}
