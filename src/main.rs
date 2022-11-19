use http::{request::Request, response::Response, Method, Status};
use parser::Parser;
use router::{Route, Router};
use std::{
	fs,
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
	sync::Arc,
	thread,
	time::Duration,
};
use thread_pool::ThreadPool;

mod error;
mod http;
mod parser;
mod result;
mod router;
mod thread_pool;

/*struct Server {
	router: Arc<Mutex<Router>>,
	pool: ThreadPool,
}

impl Server {
	fn new() -> Self {
		let router = Router::default();
		Self {
			router: Arc::new(Mutex::new(router)),
			pool: ThreadPool::new(8),
		}
	}
}*/

fn home(req: Request) -> Response {
	let content = fs::read_to_string("hello.html").unwrap();

	Response::new(*req.version(), Status::Ok, Some(&content))
}

fn hello(req: Request) -> Response {
	Response::new(*req.version(), Status::Ok, Some("hello"))
}

fn error(req: Request) -> Response {
	let content = fs::read_to_string("error.html").unwrap();

	Response::new(*req.version(), Status::Error, Some(&content))
}

fn sleep(req: Request) -> Response {
	thread::sleep(Duration::new(10, 0));

	Response::new(*req.version(), Status::Ok, None)
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

	let mut router = router::Router::default();

	let home_route = Route::new(Method::Get, "/");

	let hello_route = Route::new(Method::Get, "/hello");

	let error_route = Route::new(Method::Get, "/error");

	let sleep_route = Route::new(Method::Get, "/sleep");

	router.register(&home_route, home);
	router.register(&hello_route, hello);
	router.register(&error_route, error);
	router.register(&sleep_route, sleep);

	let router = Arc::new(router);

	let pool = ThreadPool::new(8);

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		let router = router.clone();

		pool.execute(|| {
			handle_connection(stream, router);
		});
	}
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
	let buf_reader = BufReader::new(&mut stream);

	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|r| r.unwrap())
		.take_while(|l| !l.is_empty())
		.collect();

	let http_request = Parser::raw_to_request(http_request).unwrap();

	let routes = router.routes();

	let request_handler = if let Some(route) = routes.get(&Route::from(&http_request)) {
		route
	} else {
		&routes[&Route::new(Method::Get, "/error")]
	};

	let response = request_handler(http_request);

	let response = Parser::response_to_string(response).unwrap();

	stream.write_all(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
