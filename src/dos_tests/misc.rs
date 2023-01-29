use rust_dos::*;
use rust_dos::dos::misc;

pub(crate) fn misc_test() {
    let version = misc::dos_version();

    println!("DOS version: {:?}", version);
}