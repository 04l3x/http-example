use crate::{
	http::{request::Request, response::Response, Method, Version},
	result::Result,
};

pub struct Parser;

impl Parser {
	pub fn response_to_string(response: Response) -> Result<String> {
		Ok(String::from(response))
	}

	pub fn raw_to_request(raw: Vec<String>) -> Result<Request> {
		let binding = raw[0].clone();

		let starts = binding.split(" ").collect::<Vec<_>>();

		let [method, path, version] = [starts[0], starts[1], starts[2]];

		let method = Method::try_from(method)?;

		let version = Version::try_from(version)?;

		Ok(Request::new(method, version, path))
	}
}
