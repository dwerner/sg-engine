pub mod protocol;

use self::protocol::Protocol;

struct MyProto {}

pub enum MsgType {
	One(i8),
	Two(i8, i8)
}

impl Protocol<MsgType> for MyProto {
	fn decode(&self, bytes: &[u8]) -> Result<MsgType, &'static str> {
		Err("blah")
	}
  fn encode(&self, message: &MsgType) -> Vec<u8> {
		vec![0xff, 0xff]
	}
}

#[test]
fn testing_protocol() {
	let bytes = [0xaa, 0xbb];
	let myproto = MyProto{};
	let msg = MsgType::One(8);
	myproto.encode(&msg);
}
