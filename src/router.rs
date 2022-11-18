use super::{Method, Request, Response};
use std::{collections::HashMap, fmt::Debug};

type Handler = fn(Request) -> Response;

// #[derive(Debug)]
// struct Routes<'r> {
//     method: Method,
//     routes: HashMap<&'r str, Handler>,
// }

#[derive(Debug, Default)]
pub struct Router<'r> {
    get: HashMap<&'r str, Handler>,
    post: HashMap<&'r str, Handler>,
}

impl<'r> Router<'r> {
    pub(crate) fn register(&mut self, route: &'r Route, handler: Handler) {
        match route.method() {
            Method::Get => self.get.entry(route.path()).or_insert(handler),
            Method::Post => self.post.entry(route.path()).or_insert(handler),
        };
    }

    pub fn get(&self) -> &HashMap<&'r str, Handler> {
        &self.get
    }

    pub fn post(&self) -> &HashMap<&'r str, Handler> {
        &self.post
    }
}

#[derive(Debug)]
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

    fn method(&self) -> &Method {
        &self.method
    }

    fn path(&self) -> &String {
        &self.path
    }
}
