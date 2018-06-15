use std::sync::RwLock;

use rocket::State;

use ::NightbotHeaderFields;
use data::GameData;

#[derive(FromForm)]
pub struct CivRequestInfo {
	civ: String
}

enum CivTechResult {
	Yes(String, String),
	No(String, String),
	InvalidTech,
	InvalidCiv
}

fn fetch_tech_data<S>(data: &GameData, name: S) -> Option<String> where S: AsRef<str> {
	let tech = data.tech_by_name(&name);
	
	tech.map(|tech| {
		let unit = data.unit_by_name(&name);
		let description = if !tech.for_what.is_empty() && unit.is_none() {
			format!(" Effects: {}.", tech.for_what)
		} else {
			String::new()
		};
		
		format!("{} ({}) costs {}, takes {} to research.{}", tech.name, tech.type_name, tech.cost, tech.time, description)
	})
}

fn fetch_civ_has_tech<S, T>(data: &GameData, tech_name: S, civ: T) -> CivTechResult where S: AsRef<str>, T: AsRef<str> {
	let civ = civ.as_ref();
	let tech = data.tech_by_name(tech_name);
	
	if let Some(tech) = tech {
		let civ_name = tech.available_to.iter().filter(|c| c.eq_ignore_ascii_case(civ)).next();
		let civ_name_2 = tech.not_available_to.iter().filter(|c| c.eq_ignore_ascii_case(civ)).next();
		
		if let Some(civ_name) = civ_name {
			CivTechResult::Yes(tech.name.clone(), civ_name.to_string())
		} else if let Some(civ_name) = civ_name_2 {
			CivTechResult::No(tech.name.clone(), civ_name.to_string())
		} else {
			CivTechResult::InvalidCiv
		}
	} else {
		CivTechResult::InvalidTech
	}
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
	let user_name = nightbot_headers.user.and_then(|user_params| ::parse_nightbot_user_param(user_params, "displayName"));
	let mention = if let Some(user_name) = user_name {
		format!("@{}: ", user_name)
	} else {
		String::new()
	};
	let tech_info = fetch_tech_data(&data, &name).unwrap_or(String::from("That tech does not exist."));
	
	format!("{}{}", mention, tech_info)
}

/*
 * Request handler for the tech resource with query parameters.
 * Checks if the specified civ has that tehch.
 * "data_lock" is the GameData struct kept persistent between requests by Rocket.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/tech/<name>?<civ>")]
pub fn tech_with_query_params(data_lock: State<RwLock<GameData>>, name: String, civ: CivRequestInfo, nightbot_headers: NightbotHeaderFields) -> String {
	let data = data_lock.read().unwrap();
	let user_name = nightbot_headers.user.and_then(|user_params| ::parse_nightbot_user_param(user_params, "displayName"));
	let mention = if let Some(user_name) = user_name {
		format!("@{}: ", user_name)
	} else {
		String::new()
	};
	
	let tech_info = match fetch_civ_has_tech(&data, &name, &civ.civ) {
		CivTechResult::Yes(tech, civ) => format!("{} do have {}!", civ, tech),
		CivTechResult::No(tech, civ) => format!("{} do not have {}.", civ, tech),
		CivTechResult::InvalidTech => String::from("That tech does not exist."),
		CivTechResult::InvalidCiv => String::from("That civ does not exist.")
	};
	
	format!("{}{}", mention, tech_info)
}
