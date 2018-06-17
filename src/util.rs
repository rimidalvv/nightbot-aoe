use std::collections::HashMap;

use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome as RequestOutcome
	},
	outcome::Outcome,
	http::Status
};
use url::form_urlencoded;

/*
 * The header fields Nightbot passes with each request.
 */
#[allow(dead_code)]
pub struct NightbotHeaderFields {
	pub response_url: String,
	pub user: Option<String>,
	pub channel: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for NightbotHeaderFields {
	type Error = ();
	
	fn from_request(request: &'a Request<'r>) -> RequestOutcome<Self, Self::Error> {
		let headers = request.headers();
		
		if let (Some(response_url), Some(channel)) = (headers.get_one("Nightbot-Response-Url"), headers.get_one("Nightbot-Channel")) {
			let nightbot_headers = NightbotHeaderFields {
				response_url: response_url.to_string(),
				user: headers.get_one("Nightbot-User").map(ToString::to_string),
				channel: channel.to_string()
			};
			
			Outcome::Success(nightbot_headers)
		} else if cfg!(debug_assertions) {
			let nightbot_headers = NightbotHeaderFields {
				response_url: String::from("[Nightbot response URL]"),
				user: Some(String::from("[Nightbot user header]")),
				channel: String::from("[Nightbot channel]")
			};
			
			Outcome::Success(nightbot_headers)
		} else {
			Outcome::Failure((Status::Forbidden, ()))
		}
	}
}

/*
 * Parses the the Nightbot user header fields.
 */
pub fn parse_nightbot_user_param<S>(params: S) -> HashMap<String, String> where S: AsRef<str> {
	let params = params.as_ref();
	let mut map = HashMap::new();
	
	for (k, v) in form_urlencoded::parse(params.as_bytes()) {
		map.insert(k.to_string(), v.to_string());
	}
	
	map
}

/*
 * Creates a response which mentions the user that issues the request.
 */
pub fn create_response<S>(response: S, nightbot_headers: &NightbotHeaderFields) -> String where S: AsRef<str> {
	let user_name = nightbot_headers.user.as_ref()
		.and_then(|user_param| {
			let mut params = parse_nightbot_user_param(user_param);
			
			params.remove("displayName")
		});
	let mention = if let Some(user_name) = user_name {
		format!("@{}: ", user_name)
	} else {
		String::new()
	};
	
	format!("{}{}", mention, response.as_ref())
}

/*
 * Removes spaces, dashes and leading s' from a string.
 */
pub fn shrink(s: &str) -> String {
	let mut s = s.trim().replace(' ', "").replace('-', "");
	
	if let Some('s') = s.chars().last() {
		s.pop();
	}
	
	s
}
