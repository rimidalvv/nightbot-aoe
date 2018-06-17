use std::sync::RwLock;

use rocket::State;

use util::{self, NightbotHeaderFields};
use data::GameData;

/*
 * Result when checking if a civ has an entity.
 */
enum AvailableResult {
	Yes(String, String),
	No(String, String),
	InvalidCiv,
	InvalidEntity
}

/*
 * Return plural form of units / buildings and make it lowercase.
 */
fn plural_lowercase(mut s: String) -> String {
	s = s.to_lowercase();
	
	if s.ends_with("man") {
		s.pop();
		s.pop();
		s.push_str("en");
	} else if s.starts_with("man") && s != "mangonel" {
		s.remove(1);
		s.insert(1, 'e');
	} else if s.ends_with("y") {
		s.pop();
		s.push_str("ies");
	} else if s == "condottiero" {
		s = String::from("condottieri");
	} else {
		s.push('s');
	}
	
	s
}

/*
 * Checks whether the specified civ has an entity or not.
 */
fn fetch_civ_has_entity<S, T>(data: &GameData, civ: S, name: T) -> AvailableResult where S: AsRef<str>, T: AsRef<str> {
	if let Some(unit) = data.unit_by_name(&name) {
		let civ_name = unit.available_to(&civ);
		let unit_name = plural_lowercase(unit.name.clone());
		
		if let Some((true, civ_name)) = civ_name {
			AvailableResult::Yes(civ_name, unit_name)
		} else if let Some((false, civ_name)) = civ_name {
			AvailableResult::No(civ_name, unit_name)
		} else {
			AvailableResult::InvalidCiv
		}
	} else if let Some(building) = data.building_by_name(&name) {
		let civ_name = building.available_to(&civ);
		let building_name = plural_lowercase(building.name.clone());
		
		if let Some((true, civ_name)) = civ_name {
			AvailableResult::Yes(civ_name, building_name)
		} else if let Some((false, civ_name)) = civ_name {
			AvailableResult::No(civ_name, building_name)
		} else {
			AvailableResult::InvalidCiv
		}
	} else if let Some(tech) = data.tech_by_name(&name) {
		let civ_name = tech.available_to(&civ);
		let tech_name = tech.name.clone();
		
		if let Some((true, civ_name)) = civ_name {
			AvailableResult::Yes(civ_name, tech_name)
		} else if let Some((false, civ_name)) = civ_name {
			AvailableResult::No(civ_name, tech_name)
		} else {
			AvailableResult::InvalidCiv
		}
	} else {
		AvailableResult::InvalidEntity
	}
}

/*
 * Request handler for the availble resource.
 * Checks if the specified civ has that entity.
 * "data_lock" is the GameData struct kept persistent between requests by Rocket.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/available/<civ>/<entity>")]
pub fn available(data_lock: State<RwLock<GameData>>, civ: String, entity: String, nightbot_headers: NightbotHeaderFields) -> String {
	let data = data_lock.read().unwrap();
	
	let entity_info = match fetch_civ_has_entity(&data, &civ, &entity) {
		AvailableResult::Yes(civ, entity) => format!("{} do have {}!", civ, entity),
		AvailableResult::No(civ, entity) => format!("{} do not have {}.", civ, entity),
		AvailableResult::InvalidCiv => String::from("That civ does not exist."),
		AvailableResult::InvalidEntity => String::from("That tech / unit / building does not exist.")
	};
	
	util::create_response(entity_info, &nightbot_headers)
}
