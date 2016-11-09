extern crate discord;
extern crate libloading as lib;
extern crate dango;

use std::env;
use std::rc::Rc;
use discord::{Discord, State};
use discord::model::Message;
use discord::model::Event;
use dango::commandhandler::{Command, CommandHandler};

pub fn main() {
	let discord = Rc::new(Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expect token")
	).expect("login failed"));


	let (mut connection, ready) = discord.connect().expect("connect failed");
	println!("[Ready] {} {}", ready.user.username, ready.servers.len());

	let mut wew = CommandHandler::new("test ", discord);

	wew.load_library("wewmodule/target/release/wewmodule").unwrap();

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
				wew.handle_message(&message);
			}
			_ => {},
		}
	}
}