use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use futures::StreamExt;
use serde::Deserialize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, value_name = "GGUF_MODEL")]
    model: PathBuf,

    /// Sets a custom config file
    #[arg(short, value_name = "LLAMMA_CPP_MAIN_EXE_PATH")]
    path: PathBuf,

    /// Sets a custom config file
    #[arg(short, value_name = "UI_PATH")]
    ui: Option<PathBuf>,
}

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
    let cli: Cli = Cli::parse();

    let app_data = web::Data::new(AppState {
        bin_path: match fs::canonicalize(cli.path) {
            Ok(full_path) => full_path.display().to_string(),
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        },
        model_path: match fs::canonicalize(cli.model) {
            Ok(full_path) => full_path.display().to_string(),
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        },
    });

    if let Some(ui) = &cli.ui {
        println!(
            "Serving UI on http://localhost:8080/ui/ from {}",
            match fs::canonicalize(ui) {
                Ok(full_path) => full_path.display().to_string(),
                Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
            }
        );
    }
    println!(
        r#"Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/text-completion?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/text-completion"#
    );

    HttpServer::new(move || {
        let mut a = App::new().app_data(app_data.clone()).service(
            web::resource("/text-completion")
                .route(web::get().to(handle_get))
                .route(web::post().to(handle_post)),
        );

        if let Some(ui_path) = &cli.ui {
            a = a.service(
                actix_files::Files::new(
                    "/ui",
                    match fs::canonicalize(ui_path) {
                        Ok(full_path) => full_path.display().to_string(),
                        Err(err) => {
                            panic!("Failed to serve UI: {}", err)
                        }
                    },
                )
                .index_file("index.html"),
            );
        }

        a
    })
    .bind("localhost:8080")?
    .run()
    .await
}

fn execute_llm(
    bin_path: &str,
    model_path: &str,
    prompt: String,
    sender: mpsc::UnboundedSender<String>,
) {
    let prompt = format!("\"{}\"", prompt);
    let vec_cmd = vec![
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

    let mut child = Command::new(bin_path)
        .args(&vec_cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let stdout = BufReader::new(child.stdout.take().unwrap());

    for line in stdout.lines().flatten() {
        sender.send(line).unwrap();
    }
}
