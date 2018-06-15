#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod request;
mod voobly;
mod elo;
mod data;
mod tech;

use std::env;
use std::sync::RwLock;

use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome as RequestOutcome
	},
	outcome::Outcome,
	http::Status
};

use voobly::VooblyApi;
use data::GameData;

const BUILDING_DATA: &'static str = include_str!("../res/data/buildings.json");
const CIV_DATA: &'static str = include_str!("../res/data/civs.json");
const TECH_DATA: &'static str = include_str!("../res/data/techs.json");
const UNIT_DATA: &'static str = include_str!("../res/data/units.json");

/*
 * The header fields Nightbot passes with each request.
 */

#[allow(dead_code)]
pub struct NightbotHeaderFields {
	response_url: String,
	user: Option<String>,
	channel: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for NightbotHeaderFields {
	type Error = ();
	
	fn from_request(request: &'a Request<'r>) -> RequestOutcome<Self, Self::Error> {
		let headers = request.headers();
		
		if let (Some(response_url), Some(channel)) = (headers.get_one("Nightbot-Response-Url"), headers.get_one("Nightbot-Channel")) {
			let nightbot_headers = NightbotHeaderFields {
				response_url: response_url.to_string(),
				user: headers.get_one("Nightbot-User").map(ToString::to_string),
				channel: channel.to_string()
			};
			
			Outcome::Success(nightbot_headers)
		} else if cfg!(debug_assertions) {
			let nightbot_headers = NightbotHeaderFields {
				response_url: String::from("[Nightbot response URL]"),
				user: Some(String::from("[Nightbot user header]")),
				channel: String::from("[Nightbot channel]")
			};
			
			Outcome::Success(nightbot_headers)
		} else {
			Outcome::Failure((Status::Forbidden, ()))
		}
	}
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
		.mount("/", routes![elo::elo, tech::tech, tech::tech_with_query_params])
		.launch();
}

/*
 * Parses the the Nightbot user header field
 */
pub fn parse_nightbot_user_param<S, T>(params: S, key: T) -> Option<String> where S: AsRef<str>, T: AsRef<str> {
	let params = params.as_ref();
	let target_key = key.as_ref();
	
	for kv_pair in params.split("&") {
		let mut kv = kv_pair.split("=").take(2);
		let key = kv.next();
		let val = kv.next();
		
		if key == Some(target_key) {
			return val.map(ToString::to_string);
		}
	}
	
	None
}
