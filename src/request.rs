use reqwest::{
	Client,
	IntoUrl,
	Response
};

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
