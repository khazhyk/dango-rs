extern crate libloading;
extern crate discord;

use error::Result;
use std::rc::Rc;
use discord::Discord;
use discord::model::Message;
use commands::{Command, CommandHandler};

pub struct ModuleConfig {
	pub commands: Vec<Command>,
	pub command_handlers: Vec<CommandHandler>,
	//features: Vec<Feature>,
	//deps: Vec<Feature>,
}

pub struct Module {
	library: libloading::Library,  // This must stay alive as long as any commands are alive.
	commands: Vec<Rc<Command>>
}

pub struct Bot {
	discord: Rc<Discord>,
	root_command_handler: CommandHandler,
	modules: Vec<Module>,
	//command_handlers: Vec<CommandHandler>,
}

impl Bot {
	pub fn new(discord: Discord) -> Bot {
		let d = Rc::new(discord);
		Bot {
			discord: d.clone(),
			root_command_handler: CommandHandler::new("!?", d.clone()),
			modules: vec![],
		}
	}

	pub fn load_library<S: Into<String>>(&mut self, name: S) -> Result<()> {
		let library = try!(libloading::Library::new(name.into()));

		let config = unsafe {
			let get_config = try!(library.get::<unsafe extern fn() -> ModuleConfig>(b"get_config"));
			get_config()
		};

		
		let new_module = Module {
			library: library,
			commands: config.commands.into_iter().map(|x| Rc::new(x)).collect()
		};

		for cmd in &new_module.commands {
			self.root_command_handler.register(cmd.clone());
		}

		self.modules.push(new_module);
		Ok(())
	}

	pub fn unload_library<S: Into<String>>(&mut self, name: S) -> Result<()> {
		Ok(())
	}

	pub fn handle_message(&self, message: &Message) {
		self.root_command_handler.handle_message(&message); // TODO - fall thru
	}
}