pub mod state;

// this method is called for each iteration of the loop
// the idea here is to provide a simple bridge between the shell application
#[no_mangle]
pub extern "C" fn use_state( s: &mut state::State ) {
	println!("state: ooh wow wow  {} {}", s.name, s.blob);
	if s.blob % 100 == 0 {
		println!("woot");
	}

	if s.blob % 3 == 0 {
		println!("threesies!");
	}

	if s.blob % 5 == 0 {
		println!("five and dime!");
	}
	s.blob += 1;
}
