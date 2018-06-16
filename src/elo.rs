use std::sync::RwLock;

use rocket::State;

use util::{self, NightbotHeaderFields};
use voobly::VooblyApi;

/*
 * Possible query parameters passed to the elo resource.
 */
#[derive(FromForm)]
pub struct VooblyLadderInfo {
	ladder: Option<String>
}

/*
 * Parses passed ladder into (ladder id, canonical name).
 */
fn parse_ladder(ladder: &VooblyLadderInfo) -> (String, String) {
	let (ladder, canonical) = match ladder.ladder.as_ref().map(String::as_str) {
		Some("rmtg") => (VooblyApi::RM_TG, "RM TG"),
		Some("dm1v1") => (VooblyApi::DM_1_V_1, "DM 1v1"),
		Some("dmtg") => (VooblyApi::DM_TG, "DM TG"),
		_ => (VooblyApi::RM_1_V_1, "RM 1v1")
	};
	
	(ladder.to_string(), canonical.to_string())
}

/*
 * Fetches the elo of a player.
 * If the player doesn't exist, None is returned.
 * If the player is not rated, Some(None, ..., ..., ...) is returned.
 * Voobly has a small tolerance for misspelled names. If the name didn't exist and Voobly guessed it, name_guessed is true.
 */
fn fetch_elo<S>(api: &mut VooblyApi, passed_name: S, ladder: VooblyLadderInfo) -> Option<(Option<String>, String, bool, String)> where S: AsRef<str> {
	let passed_name = passed_name.as_ref();
	let (ladder, ladder_canonical) = parse_ladder(&ladder);
	let (id, name) = api.user_info(passed_name)?;
	let name_guessed = !name.eq_ignore_ascii_case(passed_name);
	let elo = api.elo(id, ladder);
	
	Some((elo, name, name_guessed, ladder_canonical))
}

/*
 * Request handler for the elo resource.
 * Constructs a response based on the result of the request to the Voobly API.
 * "api_lock" is the Voobly API struct kept persistent between requests by Rocket.
 * Calls the same resource but with default query parameters "ladder=rm1v1".
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/elo/<voobly_user>")]
pub fn elo(api_lock: State<RwLock<VooblyApi>>, voobly_user: String, nightbot_headers: NightbotHeaderFields) -> String {
	let ladder = VooblyLadderInfo { ladder: Some(String::from("rm1v1")) };
	
	elo_with_query_params(api_lock, voobly_user, ladder, nightbot_headers)
}

/*
 * Request handler for the elo resource.
 * Constructs a response based on the result of the request to the Voobly API.
 * "api_lock" is the Voobly API struct kept persistent between requests by Rocket.
 * "ladder" are the query parameters (ladder). They might be None or empty.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/elo/<voobly_user>?<ladder>")]
pub fn elo_with_query_params(api_lock: State<RwLock<VooblyApi>>, voobly_user: String, ladder: VooblyLadderInfo, nightbot_headers: NightbotHeaderFields) -> String {
	let mut api = api_lock.write().unwrap();
	let elo_info = if let Some((elo, name, name_guessed, ladder_canonical)) = fetch_elo(&mut api, &voobly_user, ladder) {
		let correction = if name_guessed {
			format!("Did you mean {}? ", name)
		} else {
			String::new()
		};
		
		if let Some(elo) = elo {
			format!("{}{} is rated {} in {}.", correction, name, elo, ladder_canonical)
		} else {
			format!("{}{} is not rated in {}.", correction, name, ladder_canonical)
		}
	} else {
		String::from("That user doesn't exist.")
	};
	
	util::create_response(elo_info, &nightbot_headers)
}
