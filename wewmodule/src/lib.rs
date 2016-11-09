//lib.rs
extern crate dango;
extern crate discord;

use discord::Discord;
use discord::model::Message;
use dango::commandhandler::{CommandHandler, Command};

pub fn lad(c: &Discord, m: &Message) {
	c.send_message(&m.channel_id, "lad", "", false);
}

#[no_mangle]
pub fn setup(ch: &mut CommandHandler) {
	ch.register("wew", Command::new(lad));

	ch.register("lel", Command::new(|discord, message| {
		discord.send_message(&message.channel_id, "haha", "", false);
	}));
}
