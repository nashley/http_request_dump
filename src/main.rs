use axum::{body::Body, handler::HandlerWithoutStateExt, http::Request};
use pretty_hex::pretty_hex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	async fn request_handler(request: Request<Body>) {
		let (parts, body) = request.into_parts();
		let request_bytes = hyper::body::to_bytes(body)
			.await
			.expect("unable to collect request body into bytes");
		println!("{:#?}\nBody:\n{}", parts, pretty_hex(&request_bytes));
	}

	axum::Server::bind(
		&"0.0.0.0:8080"
			.parse()
			.expect("invalid listening address/port"),
	)
	.serve(request_handler.into_make_service())
	.await
	.unwrap();
}
