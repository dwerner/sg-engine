pub mod state;

// this method is called for each iteration of the loop
// the idea here is to provide a simple bridge between the shell application
#[no_mangle]
//#[cfg(crate_type="dylib")]
pub extern "C" fn use_state( s: &mut state::State ) {
    println!("Hello from dylib :P! {} {} ------ ahhhhahahhahha", s.name, s.blob);
    s.blob += 3;
}
