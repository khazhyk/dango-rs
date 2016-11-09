extern crate libloading;

use std::collections::HashMap;
use std::vec::Vec;
use discord::{Discord};
use discord::model::Message;
use std::rc::Rc;
use std::io;

pub struct Context {
	discord: Rc<Discord>,
	handler: CommandHandler,
}

pub struct Command {
	invoke: Box<Fn(&Discord, &Message)>
}

impl Command {
	pub fn new<F>(invoke: F) -> Command
		where F: 'static + Fn(&Discord, &Message) {
		Command {
			invoke: Box::new(invoke)
		}
	}
}

pub struct CommandHandler {
	command_prefix: String,
	commands: HashMap<String, Command>,
	discord: Rc<Discord>,
	libraries: Vec<libloading::Library>,
}

impl CommandHandler {
	pub fn new<S: Into<String>>(command_prefix: S,
								discord: Rc<Discord>) -> CommandHandler {
		CommandHandler {
			command_prefix: command_prefix.into(),
			discord: discord,
			commands: HashMap::new(),
			libraries: Vec::new(),
		}
	}

	pub fn load_library<S: Into<String>>(&mut self, name: S) -> Result<(), io::Error> {
		let lib = try!(libloading::Library::new(name.into()));
		unsafe {
			let setup: libloading::Symbol<unsafe extern "C" fn(&mut CommandHandler) > = try!(lib.get(b"setup"));
			setup(self);
		}
		self.libraries.push(lib);
		Ok(())
	}

	pub fn register<S: Into<String>>(&mut self, name: S, cmd: Command) {
		self.commands.insert(name.into(), cmd);
	}

	pub fn handle_message(&self, message: &Message) -> bool {
		if message.content.starts_with(&self.command_prefix) {
			let rest = &message.content[self.command_prefix.len()..];


			match self.commands.get(&rest.to_owned()) {
				Some(command) => {
					(command.invoke)(&self.discord, &message);
					true
				},
				None => false,
			}
		} else {
			false
		}
	}
}