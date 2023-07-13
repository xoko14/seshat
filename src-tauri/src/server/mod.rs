use std::{
    env,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{
    routing::{delete, get, post},
    Router, http::Method,
};
use log::info;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::async_runtime::Sender;
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::{CorsLayer, any, Any}};

mod errors;
mod routes;

pub struct Opt {
    pub log_level: String,
    pub addr: String,
    pub port: u16,
    pub database: String,
    pub reset_database: bool,
}

pub async fn run_server(opt: Opt, rx: Sender<String>) {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }

    tracing_subscriber::fmt::init();

    if opt.reset_database {
        if Sqlite::database_exists(&opt.database)
            .await
            .unwrap_or(false)
        {
            Sqlite::drop_database(&opt.database).await.unwrap();
            info!("Database dropped!")
        }
    }

    if !Sqlite::database_exists(&opt.database)
        .await
        .unwrap_or(false)
    {
        match Sqlite::create_database(&opt.database).await {
            Ok(_) => info!("Database created!"),
            Err(e) => panic!("error: {}", e),
        }
    } else {
        info!("Database found!")
    }

    let db_pool = SqlitePool::connect(&opt.database).await.unwrap();

    match sqlx::query(include_str!("sql/create.sql"))
        .execute(&db_pool)
        .await
    {
        Ok(_) => info!("Database initialized!"),
        Err(e) => panic!("Error initializing database: {}", e),
    }

    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
    .allow_origin(Any);

    let app = Router::new()
        .route("/test", get(check_conn))
        .route(
            "/sellers",
            post(routes::create_seller).get(routes::get_sellers),
        )
        .route("/sellers/:id", delete(routes::delete_seller))
        .route(
            "/articles",
            post(routes::create_article).get(routes::get_articles),
        )
        .route("/articles/:code", get(routes::get_article))
        .route("/sale", post(routes::new_sale))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()).layer(cors))
        .with_state(db_pool);

    let addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    info!("listening on http://{}", addr);

    _ = rx.send("OK".to_owned()).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn check_conn() -> String {
    "ok".to_owned()
}
