//lib.rs
extern crate dango;
extern crate discord;

use dango::error::Result;
use dango::commands::Context;
use dango::commands::Command;
use dango::module::ModuleRegistration;

pub fn lad(c: &Context) -> Result<()> {
	try!(c.say("lad"));
	Ok(())
}

#[no_mangle]
pub fn setup(reg: &mut ModuleRegistration) {
	reg.register_command(Command::new("wew", lad));

	reg.register_command(Command::new("lel", |context| {
		try!(context.say("haha"));
		Ok(())
	}));

	reg.register_command(Command::new("newcmd", |context| {
		try!(context.say("without restarting"));
		Ok(())
	}));
}
