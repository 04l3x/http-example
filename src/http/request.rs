use crate::http::{Method, Version};

#[derive(Debug)]
pub struct Request {
	method: Method,
	version: Version,
	path: String,
}

impl Request {
	pub fn new(method: Method, version: Version, path: &str) -> Self {
		Self {
			method,
			version,
			path: path.to_owned(),
		}
	}

	pub fn method(&self) -> &Method {
		&self.method
	}

	pub fn version(&self) -> &Version {
		&self.version
	}

	pub fn path(&self) -> &String {
		&self.path
	}
}
