use std::collections::HashMap;

use request;

pub struct VooblyApi {
	key: String,
	id_cache: HashMap<String, (String, String)>
}

impl VooblyApi {
	pub fn new<S>(key: S) -> Self where S: Into<String> {
		VooblyApi {
			key: key.into(),
			id_cache: HashMap::new()
		}
	}
	
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
	
	pub fn elo<S>(&self, id: S) -> Option<String> where S: AsRef<str> {
		let url = format!("http://www.voobly.com/api/ladder/131?key={}&uid={}", self.key, id.as_ref());
		let response = request::get(&url)?;
		let response = parse_response(&response);
		let elo = response.get("rating")
			.map(ToString::to_string);
		
		elo
	}
}

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
