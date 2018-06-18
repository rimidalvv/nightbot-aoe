use std::collections::HashMap;

use reqwest::{
	Client,
	ClientBuilder,
	IntoUrl,
	Response,
	RedirectPolicy,
	header::{
		Cookie as CookieHeader,
		SetCookie as SetCookieHeader
	}
};
use cookie::{
	Cookie,
	CookieJar
};

fn create_cookie_header(cookie_jar: &CookieJar) -> CookieHeader {
	let mut cookie_header = CookieHeader::new();
	
	for cookie in cookie_jar.iter() {
		cookie_header.append(cookie.name().to_string(), cookie.value().to_string());
	}
	
	cookie_header
}

fn update_cookies(cookie_jar: &mut CookieJar, set_cookie_header: &SetCookieHeader) {
	for set_cookie in set_cookie_header.iter() {
		let cookie = Cookie::parse(set_cookie).unwrap().into_owned();
		
		cookie_jar.add(cookie);
	}
}

/*
 * Dead simple HTTP GET request.
 */
pub fn get<U>(url: U) -> Option<String> where U: IntoUrl {
	let client = Client::new();
	
	client.get(url)
		.send()
		.ok()
		.as_mut()
		.map(Response::text)
		.and_then(Result::ok)
}

pub fn get_with_cookies<U>(url: U, cookie_jar: &mut CookieJar) -> Option<String> where U: IntoUrl {
	let client = ClientBuilder::new()
		.redirect(RedirectPolicy::none())
		.build()
		.unwrap();
	let cookie_header = create_cookie_header(cookie_jar);
	let response = client.get(url).header(cookie_header).send();
	
	if let Ok(mut response) = response {
		if let Some(set_cookie_header) = response.headers().get::<SetCookieHeader>() {
			update_cookies(cookie_jar, set_cookie_header);
		}
		
		response.text().ok()
	} else {
		None
	}
}

pub fn post_with_cookies<U, S>(url: U, cookie_jar: &mut CookieJar, form_data: Vec<(S, S)>) -> Option<String> where U: IntoUrl, S: AsRef<str> {
	let client = ClientBuilder::new()
		.redirect(RedirectPolicy::none())
		.build()
		.unwrap();
	let form_data = form_data.iter()
		.map(|(k, v)| (k.as_ref(), v.as_ref()))
		.collect::<HashMap<&str, &str>>();
	let cookie_header = create_cookie_header(cookie_jar);
	let response = client.post(url).form(&form_data).header(cookie_header).send();
	
	if let Ok(mut response) = response {
		if let Some(set_cookie_header) = response.headers().get::<SetCookieHeader>() {
			update_cookies(cookie_jar, set_cookie_header);
		}
		
		response.text().ok()
	} else {
		None
	}
}
