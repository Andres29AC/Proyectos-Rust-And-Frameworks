
/*fn main() {
    haiku_func(
        "
この道や
行く人なしに
秋の暮れ
",
    );
    haiku_func(
        "
雪の朝
二の字二の字の
下駄の跡
",
    );
    haiku_func(
        "
海にすむ
魚の如身を
月涼し
",
    );
}
*/
/*
fn main() {
    let haikus = vec![
        "この道や\n行く人なしに\n秋の暮れ",
        "雪の朝\n二の字二の字の\n下駄の跡",
        "海にすむ\n魚の如身を\n月涼し",
    ];

    if let Err(e) = haiku_func(haikus, "haikus.txt") {
        eprintln!("Error: {}", e);
    } else {
        println!("Haikus guardados exitosamente en 'haikus.txt'.");
    }
}
*/

use std::io;
use std::process;
mod functions;
use crate::functions::{haiku_from_file, haiku_func};

#[tokio::main]
async fn main() {
    println!("Ingrese el nombre del archivo de entrada:");
    let mut input_file = String::new();
    io::stdin()
        .read_line(&mut input_file)
        .expect("Error al leer la entrada.");
    let input_file = input_file.trim();

    println!("Ingrese el nombre del archivo de salida para los haikus:");
    let mut haiku_output_file = String::new();
    io::stdin()
        .read_line(&mut haiku_output_file)
        .expect("Error al leer la entrada.");
    let haiku_output_file = haiku_output_file.trim();

    println!("Ingrese el nombre del archivo de salida para las traducciones:");
    let mut translation_output_file = String::new();
    io::stdin()
        .read_line(&mut translation_output_file)
        .expect("Error al leer la entrada.");
    let translation_output_file = translation_output_file.trim();

    if let Err(e) = haiku_from_file(input_file, haiku_output_file, translation_output_file).await {
        eprintln!("Ocurrió un error: {}", e);
        process::exit(1);
    } else {
        println!("Haikus generados exitosamente en '{}'.", haiku_output_file);
        println!(
            "Traducciones generadas exitosamente en '{}'.",
            translation_output_file
        );
    }
}
