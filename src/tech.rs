use std::sync::RwLock;

use rocket::State;

use ::NightbotHeaderFields;
use data::GameData;

/*
 * Request handler for the tech resource.
 * Grabs the specified tech from the game data and processes it.
 * data_lock is the GameData struct kept persistent between requests by Rocket.
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
	let tech = data.techs.iter().filter(|t| shrink(&t.name).eq_ignore_ascii_case(&shrink(&name))).next();
	
	if let Some(tech) = tech {
		let description = if !tech.for_what.is_empty() && tech.for_what != "-" {
			format!(" Upgrades: {}.", tech.for_what)
		} else {
			String::new()
		};
		
		format!("{}{} ({}) costs {}, takes {} to research.{}", mention, tech.name, tech.type_name, tech.cost, tech.time, description)
	} else {
		format!("{}That tech does not exist.", mention)
	}
}

fn shrink(s: &str) -> String {
	s.trim().replace(' ', "").replace('-', "")
}
