use std::sync::Arc;
use tokio::{sync::Mutex, spawn};
use tokio;
use std::env;
use warp::Filter;
use maratona_animeitor_rust::data::*;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected 1 argument: {:?}", args);
        return;
    }
    let url_base = args[1].clone();

    let arc_runs = Arc::new(Mutex::new(DB::empty()));

    let shared = Arc::clone(&arc_runs);
    spawn(async move {
        let dur = tokio::time::Duration::new(1, 0);
        let mut interval = tokio::time::interval(dur);
        loop {
            interval.tick().await;
            update_runs(&url_base, shared.clone()).await;
        }        
    });

    let runs = warp::any().map(move || arc_runs.clone());

    let static_assets = warp::path("static").and(warp::fs::dir("static"));
    let runs = warp::path("runs").and(runs.clone()).and_then(serve_runs);

    let routes = static_assets.or(runs);



    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn update_runs(url_base : &String, runs : Arc<Mutex<DB>>) {
    let mut db = runs.lock().await;

    let mut runs_path = url_base.clone();
    runs_path.push_str(&"/runs".to_owned());

    let mut contest_path = url_base.clone();
    contest_path.push_str(&"/contest".to_owned());

    let mut time_path = url_base.clone();
    time_path.push_str(&"/time".to_owned());

    db.reload_time(time_path.as_str()).unwrap();
    db.reload_contest(contest_path.as_str()).unwrap();
    db.reload_runs(runs_path.as_str()).unwrap();
}

async fn serve_runs(runs : Arc<Mutex<DB>>) 
-> Result<impl warp::Reply, warp::Rejection> {
    let db = runs.lock().await;
    let r = serde_json::to_string(&*db.latest_n(10)).unwrap();
    Ok(r)
}