use std::sync::RwLock;

use rocket::State;
use time::{
	self,
	Tm,
	Duration
};

use util::{
	self,
	NightbotHeaderFields
};
use voobly::VooblyApi;

const MATCH_TIME_DIFFERENCE_THRESHOLD_HOURS: i64 = 3;

struct MatchData {
	time: Tm,
	winners: Vec<(Option<String>, String)>,
	losers: Vec<(Option<String>, String)>
}

fn extract_names(s: &str) -> Vec<(Option<String>, String)> {
	let names_iter = s.split("<br>");
	let mut names = Vec::new();
	
	for name in names_iter {
		let mut link_start_matches = name.match_indices("\">").map(|(i, _)| i + 2);
		let mut link_end_matches = name.match_indices("</a>").map(|(i, _)| i);
		
		if let (Some(idx_start), Some(idx_end)) = (link_start_matches.next(), link_end_matches.next()) {
			if idx_start < idx_end {
				let team_or_name = &name[idx_start .. idx_end];
				
				if let (Some(idx_start), Some(idx_end)) = (link_start_matches.next(), link_end_matches.next()) {
					if idx_start < idx_end {
						let name = &name[idx_start .. idx_end];
						
						names.push((Some(team_or_name.to_string()), name.to_string()));
					}
				} else {
					names.push((None, team_or_name.to_string()));
				}
			}
		}
	}
	
	names
}

fn parse_time(s: &str) -> Option<Tm> {
	if s.starts_with("Today, ") {
		let then = &s[7 ..];
		let then = time::strptime(then, "%I:%M %P");
		
		then.ok().map(|then| {
			let mut now = time::now();
			
			now.tm_min = then.tm_min;
			now.tm_hour = then.tm_hour;
			
			now
		})
	} else if s.starts_with("Yesterday, ") {
		let then = &s[11 ..];
		let then = time::strptime(then, "%I:%M %P");
		
		then.ok().map(|then| {
			let mut now = time::now();
			let one_day = Duration::days(1);
			
			now.tm_min = then.tm_min;
			now.tm_hour = then.tm_hour;
			
			now - one_day
		})
	} else if s.ends_with(" minutes ago") {
		let then = s.split_whitespace().next().unwrap();
		let then = time::strptime(then, "%M");
		
		then.ok().map(|then| {
			let now = time::now();
			let minutes_ago = Duration::minutes(then.tm_min as i64);
			
			now - minutes_ago
		})
	} else {
		None
	}
}

fn parse_match_data<'a>(time_str: &'a str, winners_str: &'a str, losers_str: &'a str) -> Option<MatchData> {
	let time = parse_time(time_str.as_ref());
	let winners = extract_names(winners_str.as_ref());
	let losers = extract_names(losers_str.as_ref());
	
	time.map(|time| {
		MatchData {
			time: time,
			winners: winners,
			losers: losers
		}
	})
}

fn parse_score(name: &str, matches: &Vec<MatchData>) -> (u32, u32) {
	let match_time_threshold = Duration::hours(MATCH_TIME_DIFFERENCE_THRESHOLD_HOURS);
	let last_match_time = &mut time::now();
	let match_iter = matches.iter()
		.take_while(|match_data| {
			let consecutive = *last_match_time - match_time_threshold <= match_data.time;
			
			*last_match_time = match_data.time;
			
			consecutive
		});
	let mut score = 0;
	let mut score_opponent = 0;
	
	for match_data in match_iter {
		if match_data.winners.iter().filter(|w| w.1 == name).next().is_some() {
			score += 1;
		} else if match_data.losers.iter().filter(|l| l.1 == name).next().is_some() {
			score_opponent += 1;
		}
	}
	
	(score, score_opponent)
}

#[get("/score/<voobly_user>")]
pub fn score(api_lock: State<RwLock<VooblyApi>>, voobly_user: String, nightbot_headers: NightbotHeaderFields) -> String {
	let mut api = api_lock.write().unwrap();
	let name_and_matches = api.user_info(voobly_user)
		.and_then(|(id, name)| api.matches(id, 0).map(|matches| (name, matches)));
	let response = if let Some((name, matches)) = name_and_matches {
		let mut match_list = Vec::new();
		
		for match_row in matches.iter().skip(1) {
			let mut entries = match_row.iter().skip(2);
			
			if let (Some(time), Some(winners), Some(losers)) = (entries.next(), entries.next(), entries.next()) {
				if let Some(match_data) = parse_match_data(time, winners, losers) {
					match_list.push(match_data);
				}
			}
		}
		
		let (wins, losses) = parse_score(&name, &match_list);
		
		let win_answer = if wins == 1 {
			format!("{} win", wins)
		} else {
			format!("{} wins", wins)
		};
		let loss_answer = if losses == 1 {
			format!("{} loss", losses)
		} else {
			format!("{} losses", losses)
		};
		
		format!("{}, {}", win_answer, loss_answer)
	} else {
		String::from("That user doesn't exist.")
	};
	
	util::create_response(response, &nightbot_headers)
}
