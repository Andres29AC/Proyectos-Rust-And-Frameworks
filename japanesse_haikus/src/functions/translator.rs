use rust_translate::{translate};
use rust_translate::supported_languages::get_languages;


pub async fn spanish_haikus(text: &str)->Result<String,Box<dyn std::error::Error>>{
    let translated_text = translate(text,"ja","es").await?;
    Ok(translated_text)
}
