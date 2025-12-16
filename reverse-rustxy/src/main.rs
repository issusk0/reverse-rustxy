mod miku_core;

use std::thread;
use crate::miku_core::rustxy;

fn main() {
    let rustxy = thread::spawn(move || {
        rustxy::rustxy();
    });
    rustxy.join().unwrap();
}
