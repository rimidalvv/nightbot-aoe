use std::sync::RwLock;

use rocket::State;

use util::{self, NightbotHeaderFields};
use data::GameData;

/*
 * Request handler for the unit resource.
 * Grabs the specified unit from the game data and processes it.
 * "data_lock" is the GameData struct kept persistent between requests by Rocket.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/unit/<name>")]
pub fn unit(data_lock: State<RwLock<GameData>>, name: String, nightbot_headers: NightbotHeaderFields) -> String {
	let data = data_lock.read().unwrap();
	let unit_info = if let Some(unit) = data.unit_by_name(&name) {
		if !unit.range.is_empty() && unit.range != "-" {
			format!("{} ({}) costs {}, has {} HP, {} attack, {} armor and {} range.", unit.name, unit.kind, unit.cost, unit.hp, unit.attack, unit.armor, unit.range)
		} else {
			format!("{} ({}) costs {}, has {} HP, {} attack and {} armor.", unit.name, unit.kind, unit.cost, unit.hp, unit.attack, unit.armor)
		}
	} else {
		String::from("That unit does not exist.")
	};
	
	util::create_response(unit_info, &nightbot_headers)
}
