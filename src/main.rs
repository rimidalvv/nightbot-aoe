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
struct VooblyInfo {
	name: Option<String>,
	ladder: Option<String>
}

fn parse_ladder(info: &VooblyInfo) -> (String, String) {
	let (ladder, canonical) = match info.ladder.as_ref().map(String::as_str) {
		Some("rmtg") => (VooblyApi::RM_TG, "RM TG"),
		Some("dm1v1") => (VooblyApi::DM_1_V_1, "DM 1v1"),
		Some("dmtg") => (VooblyApi::DM_TG, "DM TG"),
		_ => (VooblyApi::RM_1_V_1, "RM 1v1")
	};
	
	(ladder.to_string(), canonical.to_string())
}

fn elo_response(api: &mut VooblyApi, info: VooblyInfo) -> Option<(Option<String>, String, bool, String)> {
	let (ladder, ladder_canonical) = parse_ladder(&info);
	let passed_name = info.name?;
	let (id, name) = api.user_info(&passed_name)?;
	let name_guessed = !name.eq_ignore_ascii_case(&passed_name);
	let elo = api.elo(id, ladder);
	
	Some((elo, name, name_guessed, ladder_canonical))
}

#[get("/elo?<info>")]
fn elo(api_lock: State<RwLock<VooblyApi>>, info: VooblyInfo) -> Option<String> {
	let mut api = api_lock.write().unwrap();
	let response = if let Some((elo, name, name_guessed, ladder_canonical)) = elo_response(&mut api, info) {
		if let Some(elo) = elo {
			if !name_guessed {
				format!("{} is rated {} in {}.", name, elo, ladder_canonical)
			} else {
				format!("Did you mean {}? They're rated {} in {}.", name, elo, ladder_canonical)
			}
		} else {
			if !name_guessed {
				format!("{} is not rated in {}.", name, ladder_canonical)
			} else {
				format!("Did you mean {}? They're not rated in {}.", name, ladder_canonical)
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
