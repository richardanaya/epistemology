use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Deserialize)]
struct AiPrompt {
    prompt: String,
}

struct AppState {
    bin_path: String,
    model_path: String,
}

fn execute_ai(bin_path: &str, model_path: &str, prompt: String) -> impl Responder {
    let (tx, rx) = mpsc::unbounded_channel();

    let b = bin_path.to_string().clone();
    let m = model_path.to_string().clone();
    // Spawn a thread to execute the command and send output to the channel
    thread::spawn(move || {
        execute_llm(&b, &m, prompt, tx);
    });

    // Convert the synchronous Flume receiver into an asynchronous stream
    let async_stream = UnboundedReceiverStream::from(rx)
        .map(|line| Ok::<_, actix_web::Error>(web::Bytes::from(line)));

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(async_stream)
}

async fn handle_get(data: web::Data<AppState>, query: web::Query<AiPrompt>) -> impl Responder {
    execute_ai(&data.bin_path, &data.model_path, query.prompt.clone())
}

async fn handle_post(data: web::Data<AppState>, body: String) -> impl Responder {
    execute_ai(&data.bin_path, &data.model_path, body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get model path from command line
    let model_path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} <model_path> <llama_cpp_bin_path>", std::env::args().next().unwrap());
        std::process::exit(1);
    });
    let bin_path = std::env::args().nth(2).unwrap_or_else(|| {
        println!("Usage: {} <model_path> <llama_cpp_bin_path", std::env::args().next().unwrap());
        std::process::exit(1);
    });
    let app_data = web::Data::new(AppState {bin_path, model_path });

    println!(
        r#"Listening with GET and POST on http://localhost:8080/prompt
Examples:
    * http://localhost:8080/prompt?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/prompt"#
    );

    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).service(
            web::resource("/prompt")
                .route(web::get().to(handle_get))
                .route(web::post().to(handle_post)),
        )
    })
    .bind("localhost:8080")?
    .run()
    .await
}

fn execute_llm(bin_path: &str, model_path: &str, prompt: String, sender: mpsc::UnboundedSender<String>) {
    let prompt = format!("\"{}\"", prompt);
    let vec_cmd = vec![
        "/C",
        bin_path,
        "-m",
        model_path,
        "-n",
        "128",
        "--log-disable",
        "--simple-io",
        "-e",
        "-p",
        prompt.as_str(),
    ];

    let mut child = Command::new("cmd")
        .args(&vec_cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let stdout = BufReader::new(child.stdout.take().unwrap());

    for line in stdout.lines().flatten() {
        sender.send(line).unwrap();
    }
}
