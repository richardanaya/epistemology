use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use core::panic;
use futures::StreamExt;
use serde::Deserialize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct EpistemologyCliArgs {
    #[arg(short, value_name = "GGUF_MODEL", help = "Path to GGUF model")]
    model: PathBuf,

    #[arg(
        short,
        value_name = "LLAMMA_CPP_MAIN_EXE_PATH",
        help = "Path to LLAMMA CPP main executable"
    )]
    path: PathBuf,

    #[arg(short, value_name = "UI_PATH", help = "Path to UI static files folder")]
    ui: Option<PathBuf>,

    // Output length with default 128
    #[arg(
        short,
        value_name = "OUTPUT_LENGTH",
        help = "Output length of LLM generation"
    )]
    n: Option<u32>,
}

#[derive(Deserialize)]
struct TextCompletationRequestQuery {
    prompt: String,
}

async fn handle_get(
    data: web::Data<EpistemologyCliArgs>,
    query: web::Query<TextCompletationRequestQuery>,
) -> impl Responder {
    run_streaming_llm(&data, query.prompt.clone())
}

async fn handle_post(data: web::Data<EpistemologyCliArgs>, body: String) -> impl Responder {
    run_streaming_llm(&data, body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli: EpistemologyCliArgs = EpistemologyCliArgs::parse();

    // let's make these parameters available to the web server for all requests to use
    let app_data = web::Data::new(cli.clone());

    // let's print out some helpful information for the user
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

        // let's serve the UI if the user provided a path to a static folder of files
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

fn run_streaming_llm(args: &EpistemologyCliArgs, prompt: String) -> impl Responder {
    let (tx, rx) = mpsc::unbounded_channel();

    let a = args.clone();
    // Spawn a thread to execute the command and send output to the channel
    thread::spawn(move || {
        run_llama(&a, prompt, tx);
    });

    // Convert the synchronous Flume receiver into an asynchronous stream
    let async_stream = UnboundedReceiverStream::from(rx)
        .map(|line| Ok::<_, actix_web::Error>(web::Bytes::from(line)));

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(async_stream)
}

fn run_llama(args: &EpistemologyCliArgs, prompt: String, sender: mpsc::UnboundedSender<String>) {
    let prompt = format!("\"{}\"", prompt);
    let full_model_path = match fs::canonicalize(&args.path) {
        Ok(full_path) => full_path.display().to_string(),
        Err(err) => panic!("Failed to execute AI: {}", err),
    };

    let n_str = args.n.unwrap_or(128).to_string();
    let vec_cmd = vec![
        "-m",
        &full_model_path,
        "-n",
        &n_str,
        "--log-disable",
        "--simple-io",
        "-e",
        "-p",
        prompt.as_str(),
    ];

    let mut child = Command::new(&args.model)
        .args(&vec_cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let stdout = BufReader::new(child.stdout.take().unwrap());

    for line in stdout.lines().flatten() {
        sender.send(line).unwrap();
    }
}
