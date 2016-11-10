Just playing around with rust.

## Design
Context: command invocation context.
Context {
	discord,
	message
	say: (str)
	get_feature::<Type>() -> Status<Weak<Type>, dango::Error> (of that type)
}

get_feature is weak, don't keep it around bud.

Command: command metadata + executor
Command {
	invoke: (context) -> Status<(), dango::Error>
}

CommandHandler: message -> command mapper, can register multiple (if desired)


Library loading:

LibraryConfig: commands + commandhandlers + features exposed to bot
LibraryConfig {
	commands: Vec<Command>  // Will be registered to the root command handler
	commandhandlers: Vec<CommandHandler>  // Will be registered to root message
										  // handler, w/ lower priority than
										  // root command handler. 
										  // No other modules can register here
										  // unless you expose a feature to
										  // allow for registration e.g.
	features: Vec<Feature>
	deps: Vec<Feature>  // If a library exposing this feature is reloaded, reload this one too.
}

