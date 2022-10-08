#[allow(dead_code, unused_imports, unused_variables)]
use std::fmt::Display;

mod routes;
mod types;
use handle_errors::return_error;
use routes::analysis::calculate;
use warp::Filter;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} time taken {:?} from {:?} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });
    // For Cors
    // let cors = warp::cors()
    // .allow_any_origin()
    // .allow_header("not-in-the-request")
    // .allow_methods(&[Method::PUT, Method::DELETE]);

    let ping = warp::get().map(|| "Api v0.0.1".to_string());

    let calculate = warp::post()
        .and(warp::path("calculate"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(calculate)
        .recover(return_error);

    let route = ping.or(calculate).with(log);

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await;
}
