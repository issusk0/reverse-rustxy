mod miku_core;

use std::thread;
use crate::miku_core::rustxy::{self, rustxy};

fn main() {
    let proxy = thread::spawn(move||{
        rustxy();

    });

    proxy.join();
}
