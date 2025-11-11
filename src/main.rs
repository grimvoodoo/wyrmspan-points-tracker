use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use axum::{
    Router,
    extract::Json,
    response::Html,
    routing::{get, post},
};

use serde::Deserialize;
use tower_http::services::ServeDir;

struct ScoreEntry {
    username: String,
    total: u32,
}

#[derive(Deserialize)]
struct SubmittedPlayer {
    name: String,
    total: u32,
}

fn get_scores_file_path() -> PathBuf {
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "/data".to_string());
    PathBuf::from(data_dir).join("scores.txt")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    // our router
    let app = Router::new()
        .route("/", get(show_scores))
        .route("/submit", post(submit_scores))
        .nest_service("/static", ServeDir::new("static"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_top_scores() -> Vec<ScoreEntry> {
    let mut scores = Vec::new();
    let scores_path = get_scores_file_path();
    if let Ok(contents) = fs::read_to_string(&scores_path) {
        for line in contents.lines() {
            if let Some((name, score_str)) = line.split_once(":") {
                if let Ok(total) = score_str.trim().parse::<u32>() {
                    scores.push(ScoreEntry {
                        username: name.trim().to_string(),
                        total,
                    });
                }
            }
        }
    }
    scores.sort_by(|a, b| b.total.cmp(&a.total));
    scores.truncate(5);
    scores
}

async fn show_scores() -> Html<String> {
    let scores = get_top_scores();
    let mut rows = String::new();
    for score in &scores {
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            score.username, score.total
        ));
    }
    let template = include_str!("../static/scores.html");
    let html = template.replace("<!-- {{SCORE_ROWS}} -->", &rows);
    Html(html)
}

async fn submit_scores(Json(payload): Json<Vec<SubmittedPlayer>>) -> &'static str {
    let scores_path = get_scores_file_path();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&scores_path)
        .unwrap_or_else(|_| panic!("Unable to open file '{}'", scores_path.display()));

    for player in payload {
        if !player.name.trim().is_empty() {
            let line = format!("{}: {}\n", player.name.trim(), player.total);
            if let Err(e) = file.write_all(line.as_bytes()) {
                eprint!("Error writing to file: {}", e)
            }
        }
    }
    "ok"
}
