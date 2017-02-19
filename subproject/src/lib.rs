pub mod state;

#[no_mangle]
//#[cfg(crate_type="dylib")]
pub extern "C" fn use_state( s: &mut state::State ) {
    println!("Hello from dylib :P! {} {}", s.name, s.blob);
    s.blob += 1;
}
