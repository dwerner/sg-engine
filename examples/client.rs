extern crate gameworld;
extern crate time;

const SEND_AMT:usize = 1400;

use gameworld::net::transport::Message;


fn main() {
	println!("Creating sender socket...");
	let socket: std::net::UdpSocket = match std::net::UdpSocket::bind("192.168.0.10:9091") {
		Ok(val) => val,
		Err(_) => {
			assert!(false, "Failed to bind to socket");
			return;
		}
	};
	
	let data: [u8; SEND_AMT] = [2;SEND_AMT];
	let mut total_bytes:usize = 0;
	let current_time: time::Timespec = time::now_utc().to_timespec();
	let limit = time::Duration::seconds(10);
	loop {
        std::thread::sleep_ms(1);
        let result = match socket.send_to(&data, "192.168.0.29:9090") {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to send data");
                break;
            }
        };
        if result != SEND_AMT {
            println!("Sent only {} of {}", result, SEND_AMT);
        }
        total_bytes += result;
		let iteration_time = time::now_utc().to_timespec() - current_time;
		if iteration_time >= limit {
			break;
		}
	}
	
	println!("Sent a total of {} bytes in {:?}", 
					 total_bytes, time::now_utc().to_timespec() - current_time);
	println!("dropping socket");
	drop(socket);
	println!("end of test");
}
