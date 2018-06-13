use serde_json::{
	self,
	Value,
	Map
};

pub struct GameData {
	pub buildings: Vec<Building>,
	pub civs: Vec<Civ>,
	pub gathering_data: Vec<GatheringData>,
	pub techs: Vec<Tech>,
	pub units: Vec<Unit>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Building {
	#[serde(rename = "type")]
	pub type_name: String,
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
	pub GA: String,//Something with garrisoning
	#[serde(rename = "civb")]
	pub civ_bonus: Option<Map<String, Value>>,
	pub t: String,//Same as "avail"?
	pub avail: Option<Vec<String>>,
	pub noavail: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Civ {
	pub name: String,
	pub ver: String,
	pub ct: String,
	pub uu: String,
	pub ut: String,
	pub tb: String,
	pub bs: String,
	pub tt: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GatheringData {
	#[serde(rename = "type")]
	pub type_name: String,
	pub source: String,
	pub speed: String,
	pub note: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Tech {
	#[serde(rename = "type")]
	pub type_name: String,
	pub name: String,
	pub ver: String,
	pub age: String,
	pub cost: String,
	#[serde(rename = "for")]
	pub for_name: String,
	pub time: String,
	#[serde(rename = "civb")]
	pub civ_bonus: Option<Map<String, Value>>,
	pub t: String,
	pub avail: Option<Vec<String>>,
	pub noavail: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Unit {
	#[serde(rename = "type")]
	pub type_name: String,
	pub name: String,
	pub ver: String,
	pub age: String,
	pub cost: String,
	pub bt: String,
	pub fr: String,
	pub ad: String,
	pub mr: String,
	pub los: String,
	pub hp: String,
	pub ra: String,
	pub at: String,
	pub ar: String,
	pub note: String,
	pub extra: Option<Map<String, Value>>,
	#[serde(rename = "civb")]
	pub civ_bonus: Option<Map<String, Value>>,
	pub t: String
}

impl GameData {
	pub fn new<S, T, U, V, W>(building_data: S, civ_data: T, gathering_data: U, tech_data: V, unit_data: W) -> Self where S: AsRef<str>, T: AsRef<str>, U: AsRef<str>, V: AsRef<str>, W: AsRef<str> {
		let buildings: Vec<Building> = serde_json::from_str(building_data.as_ref()).expect("Buildings JSON");
		let civs: Vec<Civ> = serde_json::from_str(civ_data.as_ref()).expect("Civs JSON");
		let gathering_data: Vec<GatheringData> = serde_json::from_str(gathering_data.as_ref()).expect("Gathering JSON");
		let techs: Vec<Tech> = serde_json::from_str(tech_data.as_ref()).expect("Tech JSON");
		let units: Vec<Unit> = serde_json::from_str(unit_data.as_ref()).expect("Unit JSON");
		
		GameData {
			buildings: buildings,
			civs: civs,
			gathering_data: gathering_data,
			techs: techs,
			units: units
		}
	}
}
