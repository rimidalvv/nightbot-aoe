use std::sync::RwLock;

use rocket::State;
use rocket::response::content;

use util::NightbotHeaderFields;
use voobly::VooblyApi;

fn fetch_matches<S>(api: &mut VooblyApi, voobly_user: S) -> Option<String> where S: AsRef<str> {
	let (id, _) = api.user_info(voobly_user)?;
	
	api.matches(id)
}

#[get("/score/<voobly_user>")]
pub fn score(api_lock: State<RwLock<VooblyApi>>, voobly_user: String, _nightbot_headers: NightbotHeaderFields) -> content::Html<String> {
	let mut api = api_lock.write().unwrap();
	let matches = fetch_matches(&mut api, voobly_user);
	
	if let Some(matches) = matches {
		content::Html(matches)
	} else {
		content::Html(String::new())
	}
}
