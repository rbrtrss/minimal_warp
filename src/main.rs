#![warn(clippy::all)]

use dotenv;
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    // dotenv::dotenv().ok();
    let db_uri = dotenv::var("DB_CONNECTION").unwrap().as_str();
    // env_logger::init();
    // // former version
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // log::warn!("this is just a warning");
    // log::info!("this is just info");
    // log::error!("Algo malio sal");

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "minimal_warp=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    // Previous version
    // let log = warp::log::custom(|info| {
    //     // Use a log macro, or slog, or println, or whatever!
    //     log::info!(
    //         "{} {} {} {:?} from {} with {:?}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
    //         info.elapsed(),
    //         info.remote_addr().unwrap(),
    //         info.request_headers()
    //     );
    // });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);

    // let store = store::Store::new();
    let store = store::Store::new(db_uri).await;
    let store_filter = warp::any().map(move || store.clone());

    // let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        // .and(id_filter)
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_questions = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_questions = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        // .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_questions = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        // .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);
    // let hello = warp::path("hello")
    //     .map(|| format!("Hola mundo!!!"));
    let routes = get_questions
        .or(add_questions)
        .or(update_questions)
        .or(delete_questions)
        .or(add_answer)
        .with(cors)
        .with(warp::trace::request())
        // .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
