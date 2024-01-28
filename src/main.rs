use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use core::panic;
use futures::StreamExt;
use gbnf::Grammar;
use openssl::pkey::PKey;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use openssl::x509::X509;
use serde::Deserialize;
use std::fs;
use std::io::BufReader;
use std::io::Read;
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
        short = 't',
        value_name = "OUTPUT_LENGTH",
        help = "Output length of LLM generation"
    )]
    tokens_max: Option<u32>,

    // Optional origin instead of localhost
    #[arg(
        short = 'o',
        value_name = "ORIGIN",
        help = "Optional origin instead of localhost"
    )]
    origin: Option<String>,

    // Port to serve on
    #[arg(short, value_name = "PORT", help = "Port to serve on")]
    port: Option<u16>,

    // HTTPS key file
    #[arg(
        short = 'k',
        value_name = "HTTPS_KEY_FILE",
        help = "HTTPS key file (optional)"
    )]
    https_key_file: Option<PathBuf>,

    // HTTPS cert file
    #[arg(
        short = 'c',
        value_name = "HTTPS_CERT_FILE",
        help = "HTTPS cert file (optional)"
    )]
    https_cert_file: Option<PathBuf>,
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

async fn css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("./index.css"))
}

async fn inter() -> impl Responder {
    let font_bytes = include_bytes!("./Inter-Thin.ttf");
    HttpResponse::Ok()
        .content_type("font/ttf")
        .body(font_bytes.to_vec())
}

async fn icon() -> impl Responder {
    let icon_bytes = include_bytes!("./icon.png");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes.to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli: EpistemologyCliArgs = EpistemologyCliArgs::parse();

    let port = cli.port.unwrap_or(8080);

    // let's make these parameters available to the web server for all requests to use
    let app_data = web::Data::new(cli.clone());

    let origin = cli.origin.unwrap_or("localhost".to_string());

    // ensure we have both key and cert if either is provided
    if cli.https_key_file.is_some() != cli.https_cert_file.is_some() {
        panic!("Must provide both HTTPS key and cert files");
    }

    let protocol = "https";

    // let's print out some helpful information for the user
    if let Some(ui) = &cli.ui {
        println!(
            "Serving UI on {}://{}:{}/ from {}",
            protocol,
            origin,
            port,
            match fs::canonicalize(ui) {
                Ok(full_path) => full_path.display().to_string(),
                Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
            }
        );
    } else {
        println!(
            "Serving UI on {}://{}:{}/ from built-in UI",
            protocol, origin, port
        );
    }
    println!(
        r#"Listening with GET and POST on {}://{}:{}/api/completion
Examples:
    * {}://{}:{}/api/completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" {}://{}:{}/api/completion
    * curl -X POST -d "robots are good" {}://{}:{}/api/embedding"#,
        protocol,
        origin,
        port,
        protocol,
        origin,
        port,
        protocol,
        origin,
        port,
        protocol,
        origin,
        port
    );

    let s = HttpServer::new(move || {
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
            a = a.route("/index.css", web::get().to(css));
            a = a.route("/Inter-Light.ttf", web::get().to(inter));
            a = a.route("/icon.png", web::get().to(icon));
        }

        a
    });

    if let (Some(key_file), Some(cert_file)) = (&cli.https_key_file, &cli.https_cert_file) {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key_file, SslFiletype::PEM)
            .unwrap();

        builder.set_certificate_chain_file(cert_file).unwrap();
        s.bind_openssl(format!("{}:{}", origin, port), builder)?
            .run()
            .await
    } else {
        let cert = rcgen::generate_simple_self_signed(vec![origin.to_owned()]).unwrap();
        let cert_file = cert.serialize_pem().unwrap();
        let key_file = cert.serialize_private_key_pem();
        let cert = X509::from_pem(cert_file.as_bytes()).unwrap();
        let key = PKey::private_key_from_pem(key_file.as_bytes()).unwrap();
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_certificate(&cert).unwrap();
        builder.set_private_key(&key).unwrap();
        s.bind_openssl(format!("{}:{}", origin, port), builder)?
            .run()
            .await
    }
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

    let n_str = args.tokens_max.unwrap_or(128).to_string();
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
    vec_cmd.push(prompt.clone());

    let mut child = Command::new(match mode {
        Mode::Completion => args.exe_path.clone(),
        Mode::Embedding => args.embedding_path.clone().unwrap(),
    })
    .args(&vec_cmd)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let child_stdout = BufReader::new(child.stdout.take().unwrap());
    const BUFFER_SIZE: usize = 1; // Set to 1 for reading one byte at a time

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, child_stdout);
    let mut buffer = [0; BUFFER_SIZE]; // A byte array buffer

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                let character = buffer[0] as char; // Convert byte to char
                sender.send(character.to_string()).unwrap(); // Send the character as a String
            }
            Err(e) => {
                eprintln!("Error reading from child process: {}", e);
                break;
            }
        }
    }
}
