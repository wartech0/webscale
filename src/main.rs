extern crate hyper;
extern crate irc;
extern crate config;
extern crate regex;

use irc::client::prelude::*;
use std::path::Path;
use config::reader;
use hyper::Client;
use regex::Regex;

fn main() {
	println!("Webscale scaling up...");

	println!("Reading configs...");

	// Configuration
	//TODO: Configuration could be much more flexible
	let conf_file = reader::from_file(Path::new("webscale.conf")).unwrap();

	let nickname = conf_file.lookup_str("webscale.nickname");
	let altnick = conf_file.lookup_str("webscale.altnick");
	let server = conf_file.lookup_str("webscale.server");
	let channel = conf_file.lookup_str("webscale.channel");

    let config = Config {
        nickname: Some(String::from(nickname.unwrap())),
        alt_nicks: Some(vec![ String::from(altnick.unwrap()), format!("wartech0r") ]),
        server: Some(String::from(server.unwrap())),
        channels: Some(vec![ String::from(channel.unwrap()) ]),
        .. Default::default()
    };

    // Setup IRC server.
	let server = IrcServer::from_config(config).unwrap();
	server.identify();

    // Setup HTTP client.
    let client = Client::new();

    // Used to catch the url's from incoming messages
    let url_pattern = Regex::new(r"(http[s]?://[^\s]+)").unwrap();

	for message in server.iter() {
		let message = message.unwrap(); //If IRC message doesn't unwrap, we probably lost connection

		print!("{}", message);

		match message.command {
			Command::PRIVMSG(ref target, ref msg) => {

                if(msg.contains("!ping")) {
                    server.send_privmsg(target, "pong");
                }

                if url_pattern.is_match(&msg) {
                    let url = url_pattern.captures(&msg).unwrap().at(0).unwrap();

                    println!("We should fetch url: {}", url);
                }

			},
			_ => (),
		}

	}

    println!("Lost connection, shutting down...");
}
