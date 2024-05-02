use echo::Echo;
use get::Get;
use ping::Ping;
use set::Set;

use crate::frame::Frame;

pub mod echo;
pub mod get;
pub mod ping;
pub mod set;

#[derive(Debug)]
pub(crate) enum Command {
    Ping(Ping),
    Echo(Echo),
    Get(Get),
    Set(Set),
    Unknown,
}

impl Command {
    pub(crate) fn parse(frame: &Frame) -> Self {
        let args = frame.to_vec();
        let cmd = args.first().map(|s| s.to_lowercase()).unwrap_or_default();

        match cmd.as_str() {
            "ping" => Command::Ping(Ping {}),
            "echo" => Command::Echo(Echo::new(args)),
            "get" => Get::new(args).map_or(Command::Unknown, Command::Get),
            "set" => Set::new(args).map_or(Command::Unknown, Command::Set),
            _ => Command::Unknown,
        }
    }
}
