extern crate libloading;
extern crate discord;

use std::collections::HashMap;
use error::Result;
use std::rc::Rc;
use std::mem;
use discord::Discord;
use discord::model::Message;
use error::Error::CommandError;
use commands::{Command, CommandHandler};

pub struct ModuleRegistration {
	commands: Vec<Command>,
	command_handlers: Vec<CommandHandler>,
}

impl ModuleRegistration {
	pub fn new() -> ModuleRegistration {
		ModuleRegistration {
			commands: vec![],
			command_handlers: vec![]
		}
	}

	pub fn register_command(&mut self, command: Command) {
		self.commands.push(command)
	}

	pub fn register_command_handler(&mut self, command_handler: CommandHandler) {
		self.command_handlers.push(command_handler)
	}
}

pub struct Module {
	library: libloading::Library,  // This must stay alive as long as any commands are alive.
	commands: Vec<Rc<Command>>
}

pub struct Bot {
	discord: Rc<Discord>,
	root_command_handler: CommandHandler,
	modules: HashMap<String, Module>,
	//command_handlers: Vec<CommandHandler>,
}

impl Bot {
	pub fn new(discord: Discord) -> Bot {
		let d = Rc::new(discord);
		let mut roothandler = CommandHandler::new("!?", d.clone());

		Bot {
			discord: d.clone(),
			root_command_handler: roothandler,
			modules: HashMap::new(),
		}
	}

	pub fn load_library<S: Into<String>>(&mut self, name: S) -> Result<()> {
		let n = name.into();
		let library = try!(libloading::Library::new(n.clone()));

		let mut registration = ModuleRegistration::new();

		unsafe {
			let setup = try!(library.get::<unsafe extern fn(&mut ModuleRegistration) >(b"setup"));
			setup(&mut registration);
		};

		
		let new_module = Module {
			library: library,
			commands: registration.commands.into_iter().map(|x| Rc::new(x)).collect()
		};

		for cmd in &new_module.commands {
			self.root_command_handler.register(cmd.clone());
		}

		self.modules.insert(n, new_module);
		Ok(())
	}

	pub fn unload_library<S: Into<String>>(&mut self, name: S) -> Result<()> {
		let n = name.into();
		let module = try!(self.modules.remove(&n).ok_or(CommandError("no")));

		for cmd in module.commands {
			self.root_command_handler.unregister(cmd.clone());
			mem::forget(Rc::try_unwrap(cmd));
		}
		
		Ok(())
	}

	pub fn handle_message(&self, message: &Message) {
		self.root_command_handler.handle_message(&message); // TODO - fall thru
	}
}