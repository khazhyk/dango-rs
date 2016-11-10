extern crate libloading;

use std::collections::HashMap;
use std::vec::Vec;
use discord::{Discord};
use discord::model::Message;
use std::rc::Rc;
use std::io;

use error::Result;
use error::Error;

pub struct Context<'a> {
	discord: Rc<Discord>,
	message: &'a Message,
}

impl<'a> Context<'a> {
	pub fn say<S: Into<String>>(&self, message: S) -> Result<Message> {
		self.discord.send_message(&self.message.channel_id, &message.into(), "", false).map_err(Error::from)
	}
}

pub type CommandInvoke = Fn(&Context) -> Result<()>;

pub struct Command {
	name: String,
	invoke: Box<CommandInvoke>
}

impl Command {
	pub fn new<S: Into<String>, F>(name: S, invoke: F) -> Command 
		where F: 'static + Fn(&Context) -> Result<()> {
		Command {
			name: name.into(),
			invoke: Box::new(invoke)
		}
	}
}

pub struct CommandHandler {
	command_prefix: String,
	commands: HashMap<String, Rc<Command>>,
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

	pub fn register(&mut self, cmd: Rc<Command>) {
		self.commands.insert(cmd.name.clone(), cmd);
	}

	// pub fn unregister(&mut self, cmd: Rc<Command>) {
		
	// }

	pub fn handle_message(&self, message: &Message) -> bool {
		if message.content.starts_with(&self.command_prefix) {
			let rest = &message.content[self.command_prefix.len()..];

			let context = Context {
				discord: self.discord.clone(),
				message: message
			};

			match self.commands.get(&rest.to_owned()) {
				Some(command) => {
					(command.invoke)(&context);
					true
				},
				None => false,
			}
		} else {
			false
		}
	}
}