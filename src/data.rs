use serde_json::{
	self,
	Value,
	Map
};

use util;

/*
 * Struct holding all the static data regarding AoE 2.
 */
pub struct GameData {
	pub buildings: Vec<Building>,
	pub civs: Vec<Civ>,
	pub techs: Vec<Tech>,
	pub units: Vec<Unit>
}

#[derive(Serialize, Deserialize)]
pub struct Civ {
	pub name: String,
	#[serde(rename = "ver")]
	pub game_version: String,
	#[serde(rename = "ct")]
	pub strength: String,
	#[serde(rename = "uu")]
	pub unique_unit: String,
	#[serde(rename = "ut")]
	pub unique_tech: String,
	#[serde(rename = "tb")]
	pub team_bonus: String,
	#[serde(rename = "bs")]
	pub boni: String
}

#[derive(Serialize, Deserialize)]
pub struct Building {
	#[serde(rename = "type")]
	pub kind: String,
	pub name: String,
	#[serde(rename = "ver")]
	pub game_version: String,
	pub age: String,
	pub cost: String,
	#[serde(rename = "bt")]
	pub building_time: String,
	pub fr: String,//???
	#[serde(rename = "los")]
	pub line_of_sight: String,
	pub hp: String,
	#[serde(rename = "ra")]
	pub range: String,
	pub at: String,//???
	#[serde(rename = "ar")]
	pub armor: String,
	#[serde(rename = "GA")]
	pub ga: String,//Something with garrisoning
	#[serde(rename = "civb")]
	pub civ_bonus: Option<Map<String, Value>>,
	pub available_to: Vec<String>,
	pub not_available_to: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Tech {
	#[serde(rename = "type")]
	pub type_name: String,
	pub name: String,
	pub game_version: String,
	pub age: String,
	pub cost: String,
	pub extra: Option<Map<String, Value>>,
	#[serde(rename = "for")]
	pub for_what: String,
	pub time: String,
	pub civ_boni: Option<Map<String, Value>>,
	pub available_to: Vec<String>,
	pub not_available_to: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Unit {
	#[serde(rename = "type")]
	pub type_name: String,
	pub name: String,
	#[serde(rename = "ver")]
	pub game_version: String,
	pub age: String,
	pub cost: String,
	#[serde(rename = "bt")]
	pub time: String,
	#[serde(rename = "fr")]
	pub attack_speed: String,
	#[serde(rename = "ad")]
	pub attack_delay: String,
	#[serde(rename = "mr")]
	pub movement_speed: String,
	#[serde(rename = "los")]
	pub line_of_sight: String,
	pub hp: String,
	#[serde(rename = "ra")]
	pub range: String,
	#[serde(rename = "at")]
	pub attack: String,
	#[serde(rename = "ar")]
	pub armor: String,
	pub note: String,
	pub extra: Option<Map<String, Value>>,
	#[serde(rename = "civb")]
	pub civ_bonus: Option<Map<String, Value>>,
	pub available_to: Vec<String>,
	pub not_available_to: Vec<String>
}

impl GameData {
	/*
	 * Parses the given JSON data into the structs above.
	 */
	pub fn new<S, T, U, V>(building_data: S, civ_data: T, tech_data: U, unit_data: V) -> Self where S: AsRef<str>, T: AsRef<str>, U: AsRef<str>, V: AsRef<str> {
		let buildings: Vec<Building> = serde_json::from_str(building_data.as_ref()).expect("Buildings JSON");
		let civs: Vec<Civ> = serde_json::from_str(civ_data.as_ref()).expect("Civs JSON");
		let techs: Vec<Tech> = serde_json::from_str(tech_data.as_ref()).expect("Tech JSON");
		let units: Vec<Unit> = serde_json::from_str(unit_data.as_ref()).expect("Unit JSON");
		
		GameData {
			buildings: buildings,
			civs: civs,
			techs: techs,
			units: units
		}
	}
	
	/*
	 * Gets a tech by name.
	 */
	pub fn tech_by_name<S>(&self, name: S) -> Option<&Tech> where S: AsRef<str> {
		let name = name.as_ref();
		
		self.techs.iter().filter(|t| util::shrink(&t.name).eq_ignore_ascii_case(&util::shrink(name))).next()
	}
	
	/*
	 * Gets a unit by name.
	 */
	pub fn unit_by_name<S>(&self, name: S) -> Option<&Unit> where S: AsRef<str> {
		let name = name.as_ref();
		
		self.units.iter().filter(|u| util::shrink(&u.name).eq_ignore_ascii_case(&util::shrink(name))).next()
	}
	
	/*
	 * Gets a building by name.
	 */
	pub fn building_by_name<S>(&self, name: S) -> Option<&Building> where S: AsRef<str> {
		let name = name.as_ref();
		
		self.buildings.iter().filter(|b| util::shrink(&b.name).eq_ignore_ascii_case(&util::shrink(name))).next()
	}
}

impl Tech {
	/*
	 * Checks if the tech is available to the specified civ.
	 */
	pub fn available_to<S>(&self, civ: S) -> Option<(bool, String)> where S: AsRef<str> {
		let civ = civ.as_ref();
		let civ_name = self.available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		let civ_name_2 = self.not_available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		
		if let Some(civ_name) = civ_name {
			Some((true, civ_name.to_string()))
		} else if let Some(civ_name) = civ_name_2 {
			Some((false, civ_name.to_string()))
		} else {
			None
		}
	}
}

impl Unit {
	/*
	 * Checks if the unit is available to the specified civ.
	 */
	pub fn available_to<S>(&self, civ: S) -> Option<(bool, String)> where S: AsRef<str> {
		let civ = civ.as_ref();
		let civ_name = self.available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		let civ_name_2 = self.not_available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		
		if let Some(civ_name) = civ_name {
			Some((true, civ_name.to_string()))
		} else if let Some(civ_name) = civ_name_2 {
			Some((false, civ_name.to_string()))
		} else {
			None
		}
	}
}

impl Building {
	/*
	 * Checks if the building is available to the specified civ.
	 */
	pub fn available_to<S>(&self, civ: S) -> Option<(bool, String)> where S: AsRef<str> {
		let civ = civ.as_ref();
		let civ_name = self.available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		let civ_name_2 = self.not_available_to.iter().filter(|c| util::shrink(&c).eq_ignore_ascii_case(&util::shrink(civ))).next();
		
		if let Some(civ_name) = civ_name {
			Some((true, civ_name.to_string()))
		} else if let Some(civ_name) = civ_name_2 {
			Some((false, civ_name.to_string()))
		} else {
			None
		}
	}
}
