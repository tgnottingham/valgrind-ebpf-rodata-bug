use libbpf_rs::skel::SkelBuilder;

mod tc {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/tc.skel.rs"));
}

use tc::*;

#[inline(never)]
fn do_work() {
    let v = std::hint::black_box(vec![1; 8 * 1024 * 1024]);
    println!("{}", v.len());
}

fn main() {
    let builder = TcSkelBuilder::default();
    let _opened_skel = builder.open().unwrap();
    do_work();
}
