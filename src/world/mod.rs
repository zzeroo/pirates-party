use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub struct World;

impl World {
    pub fn init() {
        World::run()
    }

    pub fn run() {
        println!("Open World 01 up and running!");

        let _ = thread::spawn(|| {
            loop {
                print!(".");
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        }).join();
    }
}
