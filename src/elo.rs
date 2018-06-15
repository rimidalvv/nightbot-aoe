use std::sync::RwLock;

use rocket::State;

use ::NightbotHeaderFields;
use voobly::VooblyApi;

/*
 * Possible query parameters passed to the elo resource.
 */
#[derive(FromForm)]
pub struct VooblyEloRequestInfo {
	ladder: Option<String>
}

/*
 * Parses passed ladder into (ladder id, canonical name).
 */
fn parse_ladder(info: &VooblyEloRequestInfo) -> (String, String) {
	let (ladder, canonical) = match info.ladder.as_ref().map(String::as_str) {
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
fn fetch_elo<S>(api: &mut VooblyApi, passed_name: S, info: VooblyEloRequestInfo) -> Option<(Option<String>, String, bool, String)> where S: AsRef<str> {
	let passed_name = passed_name.as_ref();
	let (ladder, ladder_canonical) = parse_ladder(&info);
	let (id, name) = api.user_info(passed_name)?;
	let name_guessed = !name.eq_ignore_ascii_case(passed_name);
	let elo = api.elo(id, ladder);
	
	Some((elo, name, name_guessed, ladder_canonical))
}

/*
 * Request handler for the elo resource.
 * Constructs a response based on the result of the request to the Voobly API.
 * "api_lock" is the Voobly API struct kept persistent between requests by Rocket.
 * "info" are the query parameters (ladder). They might be None or empty.
 * Only accepts the request if the Nightbot headers are present.
 */
#[get("/elo/<voobly_user>?<info>")]
pub fn elo(api_lock: State<RwLock<VooblyApi>>, voobly_user: String, info: VooblyEloRequestInfo, nightbot_headers: NightbotHeaderFields) -> String {
	let mut api = api_lock.write().unwrap();
	let user_name = nightbot_headers.user.and_then(|user_params| ::parse_nightbot_user_param(user_params, "displayName"));
	let mention = if let Some(user_name) = user_name {
		format!("@{}: ", user_name)
	} else {
		String::new()
	};
	let response = if let Some((elo, name, name_guessed, ladder_canonical)) = fetch_elo(&mut api, &voobly_user, info) {
		let correction = if name_guessed {
			format!("Did you mean {}? ", name)
		} else {
			String::new()
		};
		
		if let Some(elo) = elo {
			format!("{}{}{} is rated {} in {}.", mention, correction, name, elo, ladder_canonical)
		} else {
			format!("{}{}{} is not rated in {}.", mention, correction, name, ladder_canonical)
		}
	} else {
		format!("{}That user doesn't exist.", mention)
	};
	
	response
}
