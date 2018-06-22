use std::collections::HashMap;
use std::time::{
	Instant,
	Duration
};

use request;
use cookie::CookieJar;
use table_extract::Table;

/*
 * The Voobly API struct.
 */
pub struct VooblyApi {
	key: String,
	username: String,
	password: String,
	id_cache: HashMap<String, (String, String)>,
	elo_cache: HashMap<(String, String), (String, Instant)>,
	match_cache: HashMap<String, (Table, Instant)>
}

impl VooblyApi {
	pub const RM_1_V_1: &'static str = "131";
	pub const RM_TG: &'static str = "132";
	pub const DM_1_V_1: &'static str = "163";
	pub const DM_TG: &'static str = "162";
	
	const ELO_CACHE_DURATION: Duration = Duration::from_secs(180);
	const MATCH_CACHE_DURATION: Duration = Duration::from_secs(180);
	
	/*
	 * Creates a new struct with the given API key.
	 */
	pub fn new<S, T, U>(key: S, username: T, password: U) -> Self where S: Into<String>, T: Into<String>, U: Into<String> {
		VooblyApi {
			key: key.into(),
			username: username.into(),
			password: password.into(),
			id_cache: HashMap::new(),
			elo_cache: HashMap::new(),
			match_cache: HashMap::new()
		}
	}
	
	/*
	 * Fetches user display name and id by the given name.
	 */
	pub fn user_info<S>(&mut self, name: S) -> Option<(String, String)> where S: AsRef<str> {
		let name = name.as_ref();
		
		if let Some(id_name) = self.id_cache.get(&name.to_uppercase()) {
			return Some(id_name.clone());
		}
		
		let url = format!("http://www.voobly.com/api/finduser/{}?key={}", name, self.key);
		let response = request::get(&url)?;
		let response = parse_response(&response);
		let id = response.get("uid").map(ToString::to_string)?;
		let actual_name = response.get("name").map(ToString::to_string)?;
		
		if !id.is_empty() && !actual_name.is_empty() {
			self.id_cache.insert(name.to_uppercase(), (id.clone(), actual_name.clone()));
		} else {
			self.id_cache.remove(&name.to_uppercase());
		}
		
		Some((id, actual_name))
	}
	
	/*
	 * Fetches user elo by the given user id.
	 * Caches the elo for a certain amount of time.
	 */
	pub fn elo<S, T>(&mut self, id: S, ladder: T) -> Option<String> where S: AsRef<str>, T: AsRef<str> {
		let id = id.as_ref();
		let ladder = ladder.as_ref();
		let id_ladder_tuple = (id.to_uppercase(), ladder.to_uppercase());
		
		if let Some((elo, timestamp)) = self.elo_cache.remove(&id_ladder_tuple) {
			if timestamp.elapsed() < Self::ELO_CACHE_DURATION {
				self.elo_cache.insert(id_ladder_tuple, (elo.clone(), timestamp));
				
				return Some(elo);
			}
		}
		
		let url = format!("http://www.voobly.com/api/ladder/{}?key={}&uid={}", ladder, self.key, id);
		let response = request::get(&url)?;
		let response = parse_response(&response);
		let elo = response.get("rating").map(ToString::to_string);
		
		if let Some(elo) = elo.clone() {
			self.elo_cache.insert(id_ladder_tuple, (elo, Instant::now()));
		}
		
		elo
	}
	
	pub fn matches<S>(&mut self, id: S, page: u16) -> Option<Table> where S: AsRef<str> {
		let id = id.as_ref();
		
		if let Some((match_data, timestamp)) = self.match_cache.remove(id) {
			if timestamp.elapsed() < Self::MATCH_CACHE_DURATION {
				self.match_cache.insert(id.to_uppercase(), (match_data.clone(), timestamp));
				
				return Some(match_data);
			}
		}
		
		let mut cookie_jar = CookieJar::new();
		let url = format!("https://www.voobly.com/profile/view/{}/Matches/games/matches/user/{}/0/{}", id, id, page);
		let form_data = vec![("username", self.username.as_str()), ("password", self.password.as_str())];
		
		request::get_with_cookies("https://www.voobly.com", &mut cookie_jar)?;
		request::post_with_cookies("https://www.voobly.com/login/auth", &mut cookie_jar, form_data)?;
		
		let matches = request::get_with_cookies(&url, &mut cookie_jar)?;
		let match_data = Table::find_first(&matches);
		
		if let Some(match_data) = match_data.clone() {
			self.match_cache.insert(id.to_uppercase(), (match_data, Instant::now()));
		}
		
		match_data
	}
}

/*
 * Parses these weird Voobly responses into a map.
 * Responses look like this:
 * 
 * key1,key2,key3
 * val1,val2,val3
 */
fn parse_response(response: &str) -> HashMap<&str, &str> {
	let mut map = HashMap::new();
	let mut lines = response.lines().take(2);
	
	if let (Some(keys), Some(vals)) = (lines.next(), lines.next()) {
		let mut keys = keys.trim().split(",");
		let mut vals = vals.trim().split(",");
		
		while let (Some(key), Some(val)) = (keys.next(), vals.next()) {
			if !key.is_empty() && !val.is_empty() {
				map.insert(key, val);
			}
		}
	}
	
	map
}
