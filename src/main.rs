#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate url;
extern crate cookie;

#[macro_use]
extern crate serde_derive;

mod util;
mod request;
mod voobly;
mod elo;
mod data;
mod tech;
mod unit;
mod available;
mod building;

use std::env;
use std::sync::RwLock;

use voobly::VooblyApi;
use data::GameData;

const BUILDING_DATA: &'static str = include_str!("../res/data/buildings.json");
const CIV_DATA: &'static str = include_str!("../res/data/civs.json");
const TECH_DATA: &'static str = include_str!("../res/data/techs.json");
const UNIT_DATA: &'static str = include_str!("../res/data/units.json");

/*
 * Nightbot shouldn't complain if the resource doesn't exist.
 */
#[error(404)]
fn not_found() {
	;
}

/*
 * Loads the Voobly API key from the environment variable, creates a Voobly API struct and launches Rocket.
 */
fn main() {
	let api_key = if let Ok(api_key) = env::var("VOOBLY_API_KEY") {
		api_key
	} else {
		eprintln!("VOOBLY_API_KEY environment variable not set!");
		
		Default::default()
	};
	let api = VooblyApi::new(api_key);
	let api = RwLock::new(api);
	let data = GameData::new(BUILDING_DATA, CIV_DATA, TECH_DATA, UNIT_DATA);
	let data = RwLock::new(data);
	
	rocket::ignite()
		.manage(api)
		.manage(data)
		.mount("/", routes![elo::elo, elo::elo_with_ladder, tech::tech, unit::unit, available::available, building::building])
		.catch(errors![not_found])
		.launch();
}
