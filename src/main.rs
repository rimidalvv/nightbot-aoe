#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

mod voobly;
mod request;
mod elo;

use std::env;
use std::sync::RwLock;

use voobly::VooblyApi;

/*
 * Loads the Voobly API key from the environment variable, creates a Voobly API struct and launches Rocket.
 */
fn main() {
	let api_key = env::var("VOOBLY_API_KEY").unwrap();
	let api = VooblyApi::new(api_key);
	let api_lock = RwLock::new(api);
	
	rocket::ignite()
		.manage(api_lock)
		.mount("/", routes![elo::elo])
		.launch();
}
