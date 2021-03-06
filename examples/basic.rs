extern crate phoenix;
extern crate serde_json;

extern crate env_logger;

use phoenix::{Event, Phoenix};
use std::{thread, time};

pub fn main() {
    env_logger::init();

    let url = "ws://localhost:4000/socket";

    // Simulate a user
    thread::spawn(move || {
        let mut phx = Phoenix::new(url);
        let mutex_chan = phx.channel("room:lobby").clone();

        {
            let mut device_chan = mutex_chan.lock().unwrap();
            device_chan.join().expect("Error while joining channel");
        }

        loop {
            match phx.out.recv() {
                Ok(msg) => println!("user1: {:?}", msg),
                Err(_err) => (), //println!("{:?}", err)
            }
        }
    });

    thread::sleep(time::Duration::from_millis(500));

    // Simulate an other user
    let mut phx = Phoenix::new(url);
    let mutex_chan = phx.channel("room:lobby").clone();

    {
        let mut device_chan = mutex_chan.lock().unwrap();
        device_chan.join().expect("Error while joining channel");
        device_chan
            .send(
                Event::Custom("new_msg".to_string()),
                serde_json::from_str(r#"{"body": "Hello"}"#).unwrap(),
            )
            .expect("Error while sending message to server");
    }

    loop {
        match phx.out.recv() {
            Ok(msg) => println!("user2: {:?}", msg),
            Err(_err) => (), //println!("{:?}", err)
        }
    }
}
