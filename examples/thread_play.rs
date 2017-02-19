use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

extern crate time;

fn main() {
	timed_run("single_thread", thread_play1);
	timed_run("single_thread_yielding", thread_play2);
	timed_run("two_threads_passing_messages", thread_play3);
	timed_run("single_thread_as_iter", thread_play4);
	timed_run("many_threads", thread_play5);
	timed_run("children_threads", thread_play6);
	timed_run("threaded_vector", thread_play7);
}

fn timed_run<F>(name:&'static str, func:F) where F : Fn(&'static str) -> () {
	println!("{} starting", name);
	let start = time::now_utc().to_timespec();
	func(name);
	let end = time::now_utc().to_timespec();
	println!("{} ran in {:?} milliseconds\n", name,  (end - start).num_milliseconds());
}

struct <'a> ThreadActor<T:Send + 'a> {
	rx: Receiver<T>,
	tx: Sender<T>,
	handle:JoinHandle<()>
}

impl <T:Send + 'a> ThreadActor<'a, T> {
	fn new<F:Fn(T)->() + Send + 'static>(receive:F) -> ThreadActor<T> {
		let (tx, rx) = channel::<T>();
		let (tx1, rx1) = channel::<T>();
		let handle:JoinHandle<()> = thread::Builder::new().name("vec".to_string()).spawn(move || {
			loop {
				match rx.recv() {
					Ok(msg) => receive(msg),
					_ => { break; }
				};
			}
			drop(tx1);
		}).ok().expect("merp");
		ThreadActor { 
			handle: handle,
			rx: rx1,
			tx: tx
		}
	}
}

fn thread_play8() {
}

enum Plunk {
	Push(i32),
	Pop
}

fn thread_play7(name:&'static str) {
	let (tx, rx) = channel();
	let (tx1, rx1) = channel();
	let thd = thread::Builder::new().name("vec".to_string()).spawn(move || {
		let mut storage = Vec::new();
		for message in rx {
			match message {
				Plunk::Push(value) => {
					println!("vector storing {}", value);
					storage.push(value);
				},
				Plunk::Pop => {
					let popped = storage.pop();
					match popped {
						Some(i) => {
							println!("popped {}", i);
							tx1.send(i).unwrap();
						}
						None => { println!("nothing to pop!"); }
					}
				}
			};
		}
		drop(tx1);
	});
	for i in 1..1000 {
		println!("sending {} to thread with vector", i);
		tx.send(Plunk::Push(i)).unwrap();
	}
	for i in 1..1000 {
		tx.send(Plunk::Pop).unwrap();
		let stored = rx1.recv().unwrap();
		println!("{} recvd back", stored);
	}
	drop(tx);
	thd.unwrap().join();
}

fn thread_play6(name:&'static str) {
	let mut senders = Vec::new();
	let mut parent_threads = Vec::new();
	for i in 1..10 {
		let (tx, rx) = channel();
		let parent_name = format!("{}_parent_{}", name, i);
		let parent = thread::Builder::new().name(format!("{}_parent_{}", name, i).to_string()).spawn( move || {
			let value = rx.recv().unwrap();
			let mut child_senders = Vec::new();
			let mut child_threads = Vec::new();
			for j in 1..5 {
				let (ctx, crx) = channel();
				let child = thread::Builder::new().name(format!("{}->child_{}", parent_name, j).to_string()).spawn( move || {
					for x in crx {
					 println!("{} done", thread::current().name().unwrap());
					 break;
					}
				});
				child_senders.push(ctx);
				child_threads.push(child);
			}
			for child in child_senders.iter() {
				child.send( value );
			}
			for child in child_senders.iter() {
				drop(child);
			}
			for t in child_threads {
				t.unwrap().join();
			}
			println!("{} done", thread::current().name().unwrap());
		});
		senders.push(tx);
		parent_threads.push(parent);	
	}
	for sender in senders.iter() {
		sender.send(42).unwrap();
	}
	for sender in senders.iter() {
		drop(sender);
	}
	for t in parent_threads {
		t.unwrap().join();
	}
}

fn thread_play5(name: &'static str) {
	let mut threads = Vec::new();
	let mut senders = Vec::new();
	for i in 1..10_000 {
		let (tx, rx) = channel();
		let child = thread::Builder::new().name(format!("many_threads_{}", i).to_string()).spawn(move || {
			let mut i = 0;
			for x in rx {
				if x % 5_000_000 == 0 {
					println!("{}", x);
				}
				i = x;
				thread::yield_now();
			}
		});
		senders.push(tx);
		threads.push(child);
	}
	for tx in senders.iter() {
		tx.send(42).unwrap();
	}
	for tx in senders {
		drop(tx);
	}
	for child in threads {
		let _res = child.unwrap().join();
	}
}

fn thread_play4(name: &'static str) {
	let (tx, rx) = channel();
	let child1 = thread::Builder::new().name("child_b_1".to_string()).spawn(move || {
		let mut i = 0;
		// Iterating over the receiver
		for x in rx {
			if x % 500_000 == 0 {
				println!("{}", x);
			}
			i = x;
			// breaks when the channel is dropped
		}
		println!("{} died with {}", thread::current().name().unwrap(), i);
	});
	for i in 1..1_000_001 {
		tx.send(i).unwrap();
	}
	// call drop to end the channel
	drop(tx);
	let _res = child1.unwrap().join();
}

fn thread_play3(name: &'static str) {

	let (tx_1, rx_1) = channel();
	let (tx_2, rx_2) = channel();

	let child1 = thread::Builder::new().name("child_a_1".to_string()).spawn(move || {
		let mut i = 1;
		while i > 5_000_000 {
			tx_2.send(i+1);
			i = rx_2.recv().unwrap();
		}
		println!("{} died.", thread::current().name().unwrap());
	});
	
	let child2 = thread::Builder::new().name("child_a_2".to_string()).spawn(move || {
		let mut i = 1;
		while i > 5_000_00 {
			tx_1.send(i+1);
			i = rx_1.recv().unwrap();
		}
		println!("{} died.", thread::current().name().unwrap());
	});

	let _res = child1.unwrap().join();
	let _res = child2.unwrap().join();
}

fn thread_play2(name: &'static str) {
	let child = thread::Builder::new().name(name.to_string()).spawn(move || {
		let mut i = 1;
		while i > 10_000_000 {
			i += 1;
			thread::yield_now();
		}
		println!("{} died.", thread::current().name().unwrap());
	});
	let _res = child.unwrap().join();
}

fn thread_play1(name: &'static str) {
	let child = thread::Builder::new().name(name.to_string()).spawn(move || {
		let mut i = 1;
		loop {
			if i > 10_000_000 {
				break;
			}
			i += 1;
		}
		println!("{} died.", thread::current().name().unwrap());
	});
	let _res = child.unwrap().join();
}
