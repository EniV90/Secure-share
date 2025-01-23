
mod config;
mod models;
mod dto;
mod error;
mod db;
mod utils;
mod middleware;
mod handler;
mod router;

use std::sync::Arc;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue,
    Method
};
use config::Config;
use db::{
    DBClientUserRepository, DBClientFileRepository, DBClientSharedLinkRepository,
     FileRepository 
};
use router::create_router;
use sqlx::postgres::PgPoolOptions;
use tokio_cron_scheduler::{JobScheduler, Job};
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub user_repository: Arc<DBClientUserRepository>,
    pub file_repository: Arc<DBClientFileRepository>,
    pub shared_link_repository: Arc<DBClientSharedLinkRepository>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    
    dotenv().ok();

    let config = Config::init();
 
    
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                println!("Connection to database Successful! 🔌");
                pool
            }
            Err(err) => {
                println!("Failed to connect to database: {:?}", err);
                std::process::exit(1)
            }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let user_repository = Arc::new(DBClientUserRepository::new(pool.clone()));
    let file_repository = Arc::new(DBClientFileRepository::new(pool.clone()));
    let shared_link_repository = Arc::new(DBClientSharedLinkRepository::new(pool));

    let app_state = AppState {
        env: config.clone(),
        user_repository,
        file_repository: file_repository.clone(),
        shared_link_repository,
    };

    let sched = JobScheduler::new().await.unwrap();
    let job = Job::new_async("0 0 * * * *", {
        move |_, _| {
            let file_repository = file_repository.clone();
            Box::pin(async move {
                println!("Running schedule task to delete expired files...");
                if let Err(err) = file_repository.delete_expired_files().await {
                    eprintln!("Error deleting expired files: {:?}", err);
                } else {
                    println!("Successfully deleted expired files.");
                }
            })
        }
    }).unwrap();
  
    
    sched.add(job).await.unwrap();
    tokio::spawn(async move {
        sched.start().await.unwrap();
    });

    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());
    
    println!(
        "{}",
        format!("Server is running 🏃🏽‍♂️ on http://localhost:{}", config.port)
    );
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

