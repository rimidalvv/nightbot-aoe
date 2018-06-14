use serde_json::{
	self,
	Value,
	Map
};

/*
 * Struct holding all the static data regarding AoE 2.
 */
pub struct GameData {
	pub buildings: Vec<Building>,
	pub civs: Vec<Civ>,
	pub gathering_data: Vec<GatheringData>,
	pub techs: Vec<Tech>,
	pub units: Vec<Unit>
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
	pub t: String,//Same as "avail"?
	#[serde(rename = "avail")]
	pub available_to: Option<Vec<String>>,
	#[serde(rename = "noavail")]
	pub not_available_to: Option<Vec<String>>
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
	pub boni: String,
	pub tt: String//Kinda like the name, but again?
}

#[derive(Serialize, Deserialize)]
pub struct GatheringData {
	#[serde(rename = "type")]
	pub resource: String,
	pub source: String,
	pub speed: String,
	pub note: String
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
	pub kind: String,
	pub name: String,
	#[serde(rename = "ver")]
	pub game_version: String,
	pub age: String,
	pub cost: String,
	#[serde(rename = "bt")]
	pub time: String,
	pub fr: String,//???
	pub ad: String,//???
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
	pub t: String//Same as "avail"?
}

impl GameData {
	/*
	 * Parses the given JSON data into the structs above.
	 */
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
