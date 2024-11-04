use super::spanish_haikus;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufRead, BufReader, Error, Write};

pub async fn haiku_from_file(
    input_file: &str,
    haiku_output_file: &str,
    translation_output_file: &str,
) -> Result<(), Error> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut haikus: Vec<String> = Vec::new();
    let mut current_haiku = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !current_haiku.trim().is_empty() {
                haikus.push(current_haiku.clone());
                current_haiku.clear();
            }
        } else {
            current_haiku.push_str(&line);
            current_haiku.push('\n');
        }
    }
    if !current_haiku.trim().is_empty() {
        haikus.push(current_haiku);
    }

    haiku_func(haikus, haiku_output_file, translation_output_file).await
}

// pub async fn haiku_func(
//     haikus: Vec<String>,
//     haiku_output_file: &str,
//     translation_output_file: &str,
// ) -> Result<(), Error> {
//     let haiku_file = File::create(haiku_output_file)?;
//     let mut haiku_writer = BufWriter::new(haiku_file);

//     let translation_file = File::create(translation_output_file)?;
//     let mut translation_writer = BufWriter::new(translation_file);

//     for (i, haiku) in haikus.iter().enumerate() {
//         let mut lines: Vec<&str> = haiku.lines().collect();
//         lines.reverse();

//         let max_len = lines
//             .iter()
//             .map(|line| line.chars().count())
//             .max()
//             .unwrap_or(0);

//         for j in 0..max_len {
//             let mut output_line = String::new();
//             for line in &lines {
//                 let ch = line.chars().nth(j).unwrap_or(' ');
//                 output_line.push(ch);
//                 output_line.push(' ');
//             }
//             let translated_line = spanish_haikus(&output_line).await.map_err(|e| {
//                 std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
//             })?;
//             writeln!(&mut translation_writer, "{}", translated_line.trim_end())?;
//             writeln!(&mut haiku_writer, "{}", output_line.trim_end())?;
//         }

//         // Agrega un separador solo si no es el último haiku
//         if i < haikus.len() - 1 {
//             writeln!(&mut translation_writer, "------------")?;
//             writeln!(&mut haiku_writer, "------------")?;
//         }
//     }

//     Ok(())
// }
pub async fn haiku_func(
    haikus: Vec<String>,
    haiku_output_file: &str,
    translation_output_file: &str,
) -> Result<(), Error> {
    let haiku_file = File::create(haiku_output_file)?;
    let mut haiku_writer = BufWriter::new(haiku_file);

    let translation_file = File::create(translation_output_file)?;
    let mut translation_writer = BufWriter::new(translation_file);

    for (i, haiku) in haikus.iter().enumerate() {
        let lines: Vec<&str> = haiku.lines().collect();
        let max_len = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

        for j in 0..max_len {
            let mut output_line = String::new();
            let mut translated_line = String::new();

            for line in &lines {
                if let Some(ch) = line.chars().nth(j) {
                    output_line.push(ch);
                    output_line.push(' ');
                    translated_line.push(ch);
                } else {
                    output_line.push(' ');
                    translated_line.push(' ');
                }
            }

            let translated_line = spanish_haikus(&output_line).await.map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })?;

            writeln!(&mut translation_writer, "{}", translated_line.trim_end())?;
            writeln!(&mut haiku_writer, "{}", output_line.trim_end())?;
        }

        if i < haikus.len() - 1 {
            writeln!(&mut translation_writer, "------------")?;
            writeln!(&mut haiku_writer, "------------")?;
        }
    }

    Ok(())
}
