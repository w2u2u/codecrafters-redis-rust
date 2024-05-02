use std::env;

pub struct Config {
    pub port: String,
}

impl Config {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut config = Config {
            port: String::from("6379"),
        };

        for (index, arg) in args.iter().enumerate() {
            if arg == "--port" && args.len() > index {
                config.port = arg.clone();
            }
        }

        config
    }
}
