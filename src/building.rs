use std::sync::RwLock;

use rocket::State;

use util::{self, NightbotHeaderFields};
use data::GameData;

fn map_age<S>(age: S) -> String where S: AsRef<str> {
	match age.as_ref() {
		"0" => "dark age",
		"1" => "feudal age",
		"2" => "castle age",
		"3" => "imperial age",
		_ => "?"
	}.to_string()
}

/*
 * Request handler for the building resource.
 * Grabs the specified building from the game data and processes it.
 * "data_lock" is the GameData struct kept persistent between requests by Rocket.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/building/<name>")]
pub fn building(data_lock: State<RwLock<GameData>>, name: String, nightbot_headers: NightbotHeaderFields) -> String {
	let data = data_lock.read().unwrap();
	let building_info = if let Some(building) = data.building_by_name(&name) {
		let age = map_age(&building.age);
		
		if !building.range.is_empty() && building.range != "-" {
			format!("{} costs {}, is available in {}, takes {} to build and has {} range.", building.name, building.cost, age, building.building_time, building.range)
		} else {
			format!("{} costs {}, is available in {} and takes {} to build.", building.name, building.cost, age, building.building_time)
		}
	} else {
		String::from("That building does not exist.")
	};
	
	util::create_response(building_info, &nightbot_headers)
}
