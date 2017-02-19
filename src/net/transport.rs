use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{ Receiver, Sender };


// TODO: fix this old code, mostly here for reference purposes

// TODO: refer to self::protocol::PACKET_SIZE
const PACKET_SIZE:usize = 1400;

pub struct SocketActor {
	recv_thread: thread::JoinHandle<i32>,
	send_thread: thread::JoinHandle<i32>,
	send_bound: bool,
	a_rx: Receiver<Message>,
	rx: Receiver<Message>,
	tx: Sender<Message>,
}

pub enum Message {
	Ready,
	Data([u8; PACKET_SIZE], usize),
}

impl SocketActor {
	pub fn new(send_address:&'static str, recv_address:&'static str, send_to_address:&'static str) -> SocketActor {

		// send thread --> socket
		let (send_tx, send_rx) = channel();

		// send thread <-- bind
		let (a_tx, a_rx) = channel();

		// recv thread <-- socket
		let (recv_tx, recv_rx) = channel();

		let send_thread: thread::JoinHandle<i32> = thread::spawn(move || {
			println!("Attempting to bind to: {}", send_address);
			let socket = match UdpSocket::bind(send_address) {
				Ok(val) => val,
				Err(_) => panic!("unable to bind sender")
			};
			a_tx.send(Message::Ready);
			loop {
				match send_rx.recv() {
					// blindly reusing Message here
					Ok(Message::Ready) => {}
					Ok(Message::Data(data, amt)) => {
						match socket.send_to(&data, send_to_address) {
							Ok(val) => val,
							Err(_) => {
								println!("Failed to send data");
								break;
							}
						};
					},
					Err(_) => {}
				}
			}
			0
		});

		// recv thread
		let recv_thread: thread::JoinHandle<i32> = thread::spawn(move|| {
			println!("Attempting to bind to: {}", recv_address);
			let socket = match UdpSocket::bind(recv_address) {
				Ok(val) => val,
				Err(_) => panic!("unable to bind receiver")
			};	
			recv_tx.send(Message::Ready);
			loop {
				let mut buf: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
				//let (amt, src) = match socket.recv_from(&mut buf) {
				let (amt, src) = match socket.recv_from(&mut buf) {
					Ok(val) => val,
					Err(_) => {
						println!("Err on recv");
						break;
					}
				};
				recv_tx.send(Message::Data(buf, amt));
			}
			0
		});

		SocketActor { 
			send_thread: send_thread,
			recv_thread: recv_thread,
			send_bound: false,
			a_rx: a_rx,
			rx: recv_rx,
			tx: send_tx
		}
	}

	pub fn send(&mut self, data: Message) {
		// need to know that sender is bound first
		if !self.send_bound {
			match self.a_rx.recv() {
				Ok(val) => match val {
					Message::Ready => {
						println!("Send has been bound.");
						self.send_bound = true
					},
					Message::Data(_,_) => {}
				},
				Err(_) => {}
			}
		}
		self.tx.send(data);
	}

	pub fn try_recv(&self) -> Option<Message> {
		match self.rx.try_recv() {
			Ok(val) => Some(val),
			Err(_) => None
		}
	}

	pub fn join(&self) {

	}

}
