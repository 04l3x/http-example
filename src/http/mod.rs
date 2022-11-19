use crate::error::{Error, ParsingError};
use crate::result::Result;
use std::fmt::Display;

pub mod request;
pub mod response;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Method {
	Get,
	Post,
}

impl TryFrom<&str> for Method {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self> {
		match value {
			"GET" => Ok(Method::Get),
			"POST" => Ok(Method::Post),
			_ => Err(Box::new(ParsingError::BadMethod)),
		}
	}
}

impl TryFrom<String> for Method {
	type Error = Error;
	fn try_from(value: String) -> Result<Self> {
		Self::try_from(value.as_str())
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Version {
	V0_9,
	V1,
	V1_1,
	V1_2,
	V2,
	V3,
}

impl TryFrom<&str> for Version {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self> {
		match value {
			"HTTP/0.9" => Ok(Version::V0_9),
			"HTTP/1" => Ok(Version::V1),
			"HTTP/1.1" => Ok(Version::V1_1),
			"HTTP/1.2" => Ok(Version::V1_2),
			"HTTP/2" => Ok(Version::V2),
			"HTTP/3" => Ok(Version::V3),
			_ => Err(Box::new(ParsingError::BadHttpVersion)),
		}
	}
}

impl TryFrom<String> for Version {
	type Error = Error;
	fn try_from(value: String) -> Result<Self> {
		Self::try_from(value.as_str())
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let v = format!("{:?}", self);
		let v = v.replace("V", "/");
		let v = v.replace("_", ".");
		write!(f, "HTTP{}", v)
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
	Ok = 200,
	Error = 400,
}

impl Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} {:?}", *self as u32, self)
	}
}
