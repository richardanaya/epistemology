use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use core::panic;
use futures::StreamExt;
use gbnf::Grammar;
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
    exe_path: PathBuf,

    #[arg(
        short = 'd',
        value_name = "LLAMMA_CPP_EMBEDDING_EXE_PATH",
        help = "Path to LLAMMA CPP embedding executable"
    )]
    embedding_path: Option<PathBuf>,

    #[arg(
        short = 'n',
        value_name = "NUM_GPU_LAYERS",
        help = "Number of layers to delegate to GPU"
    )]
    num_layers: Option<u32>,

    #[arg(
        short,
        value_name = "GRAMMAR_PATH",
        help = "Path to grammar file (optional)"
    )]
    grammar: Option<PathBuf>,

    #[arg(
        short,
        value_name = "JSON_SCHEMA_PATH",
        help = "Path to JSON schema file to constrain output (optional)"
    )]
    json_schema: Option<PathBuf>,

    #[arg(short, value_name = "UI_PATH", help = "Path to UI static files folder")]
    ui: Option<PathBuf>,

    // Output length with default 128
    #[arg(
        short,
        value_name = "OUTPUT_LENGTH",
        help = "Output length of LLM generation"
    )]
    num_tokens_output: Option<u32>,

    // Port to serve on
    #[arg(short, value_name = "PORT", help = "Port to serve on")]
    port: Option<u16>,
}

#[derive(Deserialize)]
struct TextCompletationRequestQuery {
    prompt: String,
}

async fn handle_completion_get(
    data: web::Data<EpistemologyCliArgs>,
    query: web::Query<TextCompletationRequestQuery>,
) -> impl Responder {
    run_streaming_llm(Mode::Completion, &data, query.prompt.clone())
}

async fn handle_completion_post(
    data: web::Data<EpistemologyCliArgs>,
    body: String,
) -> impl Responder {
    run_streaming_llm(Mode::Completion, &data, body)
}

async fn handle_embedding_post(
    data: web::Data<EpistemologyCliArgs>,
    body: String,
) -> impl Responder {
    run_streaming_llm(Mode::Embedding, &data, body)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("./index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli: EpistemologyCliArgs = EpistemologyCliArgs::parse();

    let port = cli.port.unwrap_or(8080);

    // let's make these parameters available to the web server for all requests to use
    let app_data = web::Data::new(cli.clone());

    // let's print out some helpful information for the user
    if let Some(ui) = &cli.ui {
        println!(
            "Serving UI on http://localhost:{}/ from {}",
            port,
            match fs::canonicalize(ui) {
                Ok(full_path) => full_path.display().to_string(),
                Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
            }
        );
    } else {
        println!("Serving UI on http://localhost:{}/ from built-in UI", port);
    }
    println!(
        r#"Listening with GET and POST on http://localhost:{}/api/completion
Examples:
    * http://localhost:{}/api/completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" http://localhost:{}/api/completion
    * curl -X POST -d "robots are good" http://localhost:8080/api/embedding"#,
        port, port, port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _req_head| true)
            .allowed_methods(vec!["GET", "POST"]);
        let mut a = App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(
                web::resource("/api/completion")
                    .route(web::get().to(handle_completion_get))
                    .route(web::post().to(handle_completion_post)),
            )
            .service(web::resource("/api/embedding").route(web::post().to(handle_embedding_post)));

        // let's serve the UI if the user provided a path to a static folder of files
        if let Some(ui_path) = &cli.ui {
            a = a.service(
                actix_files::Files::new(
                    "/",
                    match fs::canonicalize(ui_path) {
                        Ok(full_path) => full_path.display().to_string(),
                        Err(err) => {
                            panic!("Failed to serve UI: {}", err)
                        }
                    },
                )
                .index_file("index.html"),
            );
        } else {
            a = a.route("/", web::get().to(index));
        }

        a
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}

enum Mode {
    Completion,
    Embedding,
}

fn run_streaming_llm(mode: Mode, args: &EpistemologyCliArgs, prompt: String) -> impl Responder {
    if let Mode::Embedding = mode {
        if args.embedding_path.is_none() {
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Embedding mode requires embedding path, look at help for more information");
        }
    }

    let (tx, rx) = mpsc::unbounded_channel();

    let a = args.clone();
    // Spawn a thread to execute the command and send output to the channel
    thread::spawn(move || {
        run_llama(mode, &a, prompt, tx);
    });

    // Convert the synchronous Flume receiver into an asynchronous stream
    let async_stream = UnboundedReceiverStream::from(rx)
        .map(|line| Ok::<_, actix_web::Error>(web::Bytes::from(line)));

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(async_stream)
}

fn run_llama(
    mode: Mode,
    args: &EpistemologyCliArgs,
    prompt: String,
    sender: mpsc::UnboundedSender<String>,
) {
    let full_model_path = match fs::canonicalize(&args.model) {
        Ok(full_path) => full_path.display().to_string(),
        Err(err) => panic!("Failed to execute AI: {}", err),
    };

    let n_str = args.num_tokens_output.unwrap_or(128).to_string();
    let mut vec_cmd: Vec<String> = vec![
        "-m".to_string(),
        full_model_path.to_string(),
        "-n".to_string(),
        n_str,
        "--log-disable".to_string(),
    ];

    let full_grammar_path;
    if let Some(grammar) = &args.grammar {
        full_grammar_path = match fs::canonicalize(grammar) {
            Ok(full_path) => full_path.display().to_string(),
            Err(err) => panic!("Failed to execute AI: {}", err),
        };
        vec_cmd.push("--grammar-file".to_string());
        vec_cmd.push(full_grammar_path);
    }

    if let Some(json_schema) = &args.json_schema {
        let full_json_schema_path = match fs::canonicalize(json_schema) {
            Ok(full_path) => full_path.display().to_string(),
            Err(err) => panic!("Failed to execute AI: {}", err),
        };
        let json_schema_str = fs::read_to_string(full_json_schema_path).unwrap();
        let g = Grammar::from_json_schema(&json_schema_str);
        if let Err(err) = g {
            panic!("Failed to execute AI: {}", err);
        }
        let g_str = g.unwrap().to_string();
        vec_cmd.push("--grammar".to_string());
        vec_cmd.push(g_str.to_string());
    }

    if let Some(num_layers) = &args.num_layers {
        vec_cmd.push("-ngl".to_string());
        vec_cmd.push(num_layers.to_string());
    }

    println!("Running LLM: {} ...", &vec_cmd.join(" "));

    // don't show prompt in commandline
    vec_cmd.push("-p".to_string());
    vec_cmd.push(prompt);

    let mut child = Command::new(match mode {
        Mode::Completion => args.exe_path.clone(),
        Mode::Embedding => args.embedding_path.clone().unwrap(),
    })
    .args(&vec_cmd)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let stdout = BufReader::new(child.stdout.take().unwrap());

    let lines: Vec<_> = stdout.lines().flatten().collect();
    let total_lines = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let is_last = i == total_lines - 1;

        if is_last {
            sender.send(line.clone()).unwrap();
        } else {
            sender.send(line.clone() + "\n").unwrap();
        }
    }
}
