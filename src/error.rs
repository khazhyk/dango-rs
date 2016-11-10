extern crate discord;

use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	//NoSuchFeature(),
	CommandError(&'static str),
	DiscordError(discord::Error),
	IoError(io::Error),
}

impl From<discord::Error> for Error {
	fn from(err: discord::Error) -> Error {
		Error::DiscordError(err)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::IoError(err)
	}
}
