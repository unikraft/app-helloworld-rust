#![no_std]
use ukrust;

use libm::pow;

#[no_mangle]
extern "C" fn tfoo() -> u32{
    it_works()
}

pub fn it_works() -> u32 {

    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    let x = pow(2.0, 3.0);
    x as u32
}
