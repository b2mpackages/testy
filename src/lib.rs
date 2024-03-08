#![feature(c_unwind)]

#[macro_use]
extern crate gmod;

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    println!("Test DLL module");
    0
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    0
}
