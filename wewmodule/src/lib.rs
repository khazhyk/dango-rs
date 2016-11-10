//lib.rs
extern crate dango;
extern crate discord;

use dango::error::Result;
use dango::commands::Context;
use dango::commands::Command;
use dango::module::ModuleConfig;

pub fn lad(c: &Context) -> Result<()> {
	try!(c.say("lad"));
	Ok(())
}

#[no_mangle]
pub fn get_config() -> ModuleConfig {
	ModuleConfig {
		commands: vec![
			Command::new("wew", lad),
			Command::new("lel", |context| {
				try!(context.say("haha"));
				Ok(())
			}),
		],
		command_handlers: vec![],
	}
}
