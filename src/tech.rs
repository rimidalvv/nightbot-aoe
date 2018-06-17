use std::sync::RwLock;

use rocket::State;

use util::{self, NightbotHeaderFields};
use data::GameData;

/*
 * Fetches the data for the specified tech name.
 */
fn fetch_tech_data<S>(data: &GameData, name: S) -> Option<String> where S: AsRef<str> {
	let tech = data.tech_by_name(&name);
	
	tech.map(|tech| {
		let description = if !tech.for_what.is_empty() {
			if let Some(unit) = data.unit_by_name(&tech.for_what) {
				format!(" Upgrades {}.", unit.name)
			} else if let Some(building) = data.building_by_name(&tech.for_what) {
				format!(" Upgrades {}.", building.name)
			} else {
				format!(" Effects: {}.", tech.for_what)
			}
		} else {
			String::new()
		};
		
		format!("{} ({}) costs {}, takes {} to research.{}", tech.name, tech.type_name, tech.cost, tech.time, description)
	})
}

/*
 * Request handler for the tech resource.
 * Grabs the specified tech from the game data and processes it.
 * "data_lock" is the GameData struct kept persistent between requests by Rocket.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/tech/<name>")]
pub fn tech(data_lock: State<RwLock<GameData>>, name: String, nightbot_headers: NightbotHeaderFields) -> String {
	let data = data_lock.read().unwrap();
	let tech_info = if let Some(tech_info) = fetch_tech_data(&data, &name) {
		tech_info
	} else {
		String::from("That tech does not exist.")
	};
	
	util::create_response(tech_info, &nightbot_headers)
}
