use std::collections::HashMap;
use std::time::{
	Instant,
	Duration
};

use request;

/*
 * The Voobly API struct.
 */
pub struct VooblyApi {
	key: String,
	id_cache: HashMap<String, (String, String)>,
	elo_cache: HashMap<String, (String, Instant)>
}

impl VooblyApi {
	pub const RM_1_V_1: &'static str = "131";
	pub const RM_TG: &'static str = "132";
	pub const DM_1_V_1: &'static str = "163";
	pub const DM_TG: &'static str = "162";
	
	const ELO_CACHE_DURATION: Duration = Duration::from_secs(60);
	
	/*
	 * Creates a new struct with the given API key.
	 */
	pub fn new<S>(key: S) -> Self where S: Into<String> {
		VooblyApi {
			key: key.into(),
			id_cache: HashMap::new(),
			elo_cache: HashMap::new()
		}
	}
	
	/*
	 * Fetches user display name and id by the given name.
	 */
	pub fn user_info<S>(&mut self, name: S) -> Option<(String, String)> where S: AsRef<str> {
		let name = name.as_ref();
		
		if let Some(id_name) = self.id_cache.get(name) {
			return Some(id_name.clone());
		}
		
		let url = format!("http://www.voobly.com/api/finduser/{}?key={}", name, self.key);
		let response = request::get(&url)?;
		let response = parse_response(&response);
		let id = response.get("uid").map(ToString::to_string)?;
		let actual_name = response.get("name").map(ToString::to_string)?;
		
		if !id.is_empty() && !actual_name.is_empty() {
			self.id_cache.insert(name.to_string(), (id.clone(), actual_name.clone()));
		} else {
			self.id_cache.remove(name);
		}
		
		Some((id, actual_name))
	}
	
	/*
	 * Fetches user elo by the given user id.
	 * Caches the elo for a certain amount of time.
	 */
	pub fn elo<S, T>(&mut self, id: S, ladder: T) -> Option<String> where S: AsRef<str>, T: AsRef<str> {
		let id = id.as_ref();
		
		if let Some((elo, timestamp)) = self.elo_cache.remove(id) {
			if timestamp.elapsed() < Self::ELO_CACHE_DURATION {
				self.elo_cache.insert(id.to_string(), (elo.clone(), timestamp));
				
				return Some(elo);
			}
		}
		
		let url = format!("http://www.voobly.com/api/ladder/{}?key={}&uid={}", ladder.as_ref(), self.key, id);
		let response = request::get(&url)?;
		let response = parse_response(&response);
		let elo = response.get("rating").map(ToString::to_string);
		
		if let Some(elo) = elo.clone() {
			self.elo_cache.insert(id.to_string(), (elo, Instant::now()));
		}
		
		elo
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
