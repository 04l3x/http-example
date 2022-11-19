use super::{Method, Request, Response};
use std::{collections::HashMap, fmt::Debug};

type Handler = fn(Request) -> Response;

#[derive(Debug, Default)]
pub struct Router {
	routes: HashMap<Route, Handler>,
}

impl Router {
	pub(crate) fn register(&mut self, route: &Route, handler: Handler) {
		self.routes.entry(route.clone()).or_insert(handler);
	}

	pub fn routes(&self) -> &HashMap<Route, Handler> {
		&self.routes
	}
}

impl From<&Request> for Route {
	fn from(req: &Request) -> Self {
		Self {
			method: *req.method(),
			path: req.path().clone(),
		}
	}
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Route {
	method: Method,
	path: String,
}

impl Route {
	pub(crate) fn new(method: Method, path: &str) -> Self {
		Self {
			method,
			path: path.to_string(),
		}
	}

	/*pub fn method(&self) -> &Method {
		&self.method
	}

	pub fn path(&self) -> &String {
		&self.path
	}*/
}
