extern crate dango;
extern crate discord;

use discord::Discord;
use dango::module::Bot;

#[test]
fn loading_unloading() {

	let mut bot = Bot::new(Discord::from_bot_token("").unwrap());

	bot.load_library("tests/module").unwrap();	
	bot.unload_library("tests/module").unwrap();
}