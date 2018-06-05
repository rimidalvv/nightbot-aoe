#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

mod voobly;
mod request;

use std::env;
use std::sync::RwLock;

use rocket::{
	State,
};

use voobly::VooblyApi;

#[derive(FromForm)]
struct VooblyUser {
	name: Option<String>
}

fn elo_response(api: &mut VooblyApi, user: VooblyUser) -> Option<(Option<String>, String, bool)> {
	let passed_name = user.name?;
	let (id, name) = api.user_info(&passed_name)?;
	let name_guessed = !name.eq_ignore_ascii_case(&passed_name);
	let elo = api.elo(id);
	
	Some((elo, name, name_guessed))
}

#[get("/elo?<user>")]
fn elo(api_lock: State<RwLock<VooblyApi>>, user: VooblyUser) -> Option<String> {
	let mut api = api_lock.write().unwrap();
	let response = if let Some((elo, name, name_guessed)) = elo_response(&mut api, user) {
		if let Some(elo) = elo {
			if !name_guessed {
				format!("{} is rated {}.", name, elo)
			} else {
				format!("Did you mean {}? They're {}.", name, elo)
			}
		} else {
			if !name_guessed {
				format!("{} is not rated.", name)
			} else {
				format!("Did you mean {}? They're not rated.", name)
			}
		}
	} else {
		String::from("That user doesn't exist.")
	};
	
	Some(response)
}

fn main() {
	let api_key = env::var("VOOBLY_API_KEY").unwrap();
	let api = VooblyApi::new(api_key);
	let api_lock = RwLock::new(api);
	
	rocket::ignite()
		.manage(api_lock)
		.mount("/", routes![elo])
		.launch();
}
