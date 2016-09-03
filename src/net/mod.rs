pub mod socket;

#[test]
fn test_socket() {
	use self::socket::SocketWrapper;
	let x = SocketWrapper::new(2);
	assert!(x.get_blah() == 2);
}
