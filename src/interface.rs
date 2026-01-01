// This file is an interface for the CANBus users that c++ executables
// We can read or write the counters

use crate::counter::SharedCounter;
use std::sync::{Mutex, OnceLock};
use crate::counter::GLOBAL_COUNTER;

use std::os::raw::{c_int, c_uint};



#[unsafe(no_mangle)]
pub extern "C" fn read_counter(counter_id: c_uint) -> c_int {

    let mut guard = (*GLOBAL_COUNTER).lock().expect("Could not lock mutex");

    guard.show_the_counter(counter_id as u32) as c_int

}

#[unsafe(no_mangle)]
pub extern "C" fn write_counter(counter_id: c_uint) {
    let mut guard = (*GLOBAL_COUNTER).lock().expect("Could not lock mutex");

    guard.incremet_the_counter(counter_id as u32);
}

#[unsafe(no_mangle)]
pub extern "C" fn read_nonce_value() -> c_int {
    5
}

// Important note:
// we are opening a window to the C world.
// Thus we have to use
