extern crate discord;
extern crate libloading as lib;
extern crate dango;

use std::env;
use std::rc::Rc;
use discord::{Discord, State};
use discord::model::Message;
use discord::model::Event;
use dango::module::Bot;

pub fn main() {
	let discord = Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expect token")
	).expect("login failed");


	let (mut connection, ready) = discord.connect().expect("connect failed");
	println!("[Ready] {} {}", ready.user.username, ready.servers.len());

	let mut wew = Bot::new(discord);

	wew.load_library("wewmodule/target/debug/wewmodule").unwrap();
	wew.unload_library("wewmodule/target/debug/wewmodule").unwrap();

	loop {
		let event = match connection.recv_event() {
			Ok(event) => event,
			Err(discord::Error::Closed(code, body)) => {
				println!("[Error] Connection closed with status {:?}: {}", code, body);
				break
			}
			Err(err) => {
				println!("[Warning] Receive error: {:?}", err);
				continue
			}
		};

		match event {
			Event::MessageCreate(message) => {
				if message.content == "!?load" {
					wew.load_library("wewmodule/target/debug/wewmodule").unwrap();
				} else if message.content == "!?unload" {
					wew.unload_library("wewmodule/target/debug/wewmodule").unwrap();
				}
				wew.handle_message(&message);
			}
			_ => {},
		}
	}
}