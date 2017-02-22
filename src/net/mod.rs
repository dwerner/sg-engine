pub mod socket;
pub mod protocol;
pub mod transport;

#[test]
fn test_socket() {
	use self::socket::SocketWrapper;
	let x = SocketWrapper::new(2);
	assert!(x.get_blah() == 2);
}
