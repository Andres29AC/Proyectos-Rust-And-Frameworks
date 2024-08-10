//NOTE: to use Builder API
//NOTE: -> cargo add clap --features cargo

//NOTE: to use Derive API
//NOTE: -> cargo add clap --features derive

//NOTE: Comentar en Helix C-c
//NOTE: wrap_help: Turns on the help text wrapping feature, based on the terminal size.

//NOTE: cargo add tokio --features rt-multi-thread,full
//NOTE: rt-multi-thread: Enables the heavier, multi-threaded, work-stealing scheduler.
//NOTE: full: Enables all features listed below except test-util and tracing.

//NOTE: cargo add ureq

//NOTE: Version 1
// use clap::Parser;
// #[derive(Parser, Debug)]
// #[clap(version, about)]
// struct Args {
//     #[clap(short, long)]
//     name: String,
//     #[clap(short, long, default_value = "1")]
//     ls: u8,
// }
// //NOTE: entero sin signo de 8 bits:u8
// fn main() {
//     let args = Args::parse();
//     let mut names: Vec<String> = Vec::new();
//     for _ in 0..args.ls {
//         names.push(args.name.clone());
//     }
//     println!("{}", names.join(" "));
// }
//NOTE: Version 1
// fn main() -> Result<(), ureq::Error> {
//     let body = ureq::get("https://www.upn.edu.pe/").call()?.into_string()?;
//     println!("{body}");
//     Ok(())
// }
//NOTE: Version 2
// use std::env;
// use std::fs::File;
// use std::io::Write;
// use ureq;
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Uso: {} <URL>", args[0]);
//         std::process::exit(1);
//     }
//     let url = &args[1];
//     let body = ureq::get(url).call()?.into_string()?;
//     let mut file = File::create("output.html")?;
//     file.write_all(body.as_bytes())?;
//     println!("El contenido se ha guardado en 'output.html'.");
//     Ok(())
// }

//NOTE: Version 4
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::Write;
use ureq;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Obtiene la URL desde la línea de comandos
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let url = Url::parse(&args[1])?;

    // Crea una carpeta para guardar los archivos
    let output_dir = "output_site";
    fs::create_dir_all(output_dir)?;

    // Descarga el HTML principal
    let body = ureq::get(url.as_str()).call()?.into_string()?;
    let document = Html::parse_document(&body);

    // Guarda el HTML principal
    let mut file = File::create(format!("{}/index.html", output_dir))?;
    file.write_all(body.as_bytes())?;

    // Define selectores para encontrar todos los recursos relevantes
    let selectors = vec![("img", "src"), ("link", "href"), ("script", "src")];

    // Descarga y guarda cada recurso
    for (tag, attr) in selectors {
        let selector = Selector::parse(tag)?;
        for element in document.select(&selector) {
            if let Some(resource_url) = element.value().attr(attr) {
                // Construye la URL completa
                let full_url = url.join(resource_url)?;

                // Descarga el recurso
                match ureq::get(full_url.as_str()).call() {
                    Ok(response) => {
                        if let Ok(resource_body) = response.into_string() {
                            // Construye la ruta de archivo de salida
                            let resource_path = format!("{}/{}", output_dir, resource_url);
                            let resource_path = resource_path.replace("://", "_");
                            if let Some(parent) = std::path::Path::new(&resource_path).parent() {
                                fs::create_dir_all(parent)?;
                            }
                            let mut resource_file = File::create(&resource_path)?;
                            resource_file.write_all(resource_body.as_bytes())?;
                            println!("Recurso guardado: {}", resource_url);
                        } else {
                            println!("No se pudo convertir el recurso a cadena: {}", full_url);
                        }
                    }
                    Err(_) => {
                        println!("No se pudo descargar el recurso: {}", full_url);
                    }
                }
            }
        }
    }

    println!(
        "Todos los recursos, incluido el HTML principal, se han guardado en la carpeta '{}'.",
        output_dir
    );

    Ok(())
}
