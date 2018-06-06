use std::sync::RwLock;

use rocket::State;

use voobly::VooblyApi;

/*
 * Possible query parameters passed to the elo resource.
 */
#[derive(FromForm)]
pub struct VooblyEloRequestInfo {
	name: Option<String>,
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
fn elo_response(api: &mut VooblyApi, info: VooblyEloRequestInfo) -> Option<(Option<String>, String, bool, String)> {
	let (ladder, ladder_canonical) = parse_ladder(&info);
	let passed_name = info.name?;
	let (id, name) = api.user_info(&passed_name)?;
	let name_guessed = !name.eq_ignore_ascii_case(&passed_name);
	let elo = api.elo(id, ladder);
	
	Some((elo, name, name_guessed, ladder_canonical))
}

/*
 * Request handler for the elo resource.
 * Constructs a response based on the result of the request to the Voobly API.
 * api_lock is the Voobly API struct kept persistent between requests by Rocket.
 * info are the query parameters (name and ladder). They might be None or empty.
 */
#[get("/elo?<info>")]
pub fn elo(api_lock: State<RwLock<VooblyApi>>, info: VooblyEloRequestInfo) -> Option<String> {
	let mut api = api_lock.write().unwrap();
	let response = if let Some((elo, name, name_guessed, ladder_canonical)) = elo_response(&mut api, info) {
		if let Some(elo) = elo {
			if !name_guessed {
				format!("{} is rated {} in {}.", name, elo, ladder_canonical)
			} else {
				format!("Did you mean {}? They're rated {} in {}.", name, elo, ladder_canonical)
			}
		} else {
			if !name_guessed {
				format!("{} is not rated in {}.", name, ladder_canonical)
			} else {
				format!("Did you mean {}? They're not rated in {}.", name, ladder_canonical)
			}
		}
	} else {
		String::from("That user doesn't exist.")
	};
	
	Some(response)
}
