/*
 use super::haiku_func;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_haiku_formatting() {
        // Definir el haiku de entrada y el resultado esperado
        let input_haiku = "この道や\n行く人なしに\n秋の暮れ";
        let expected_output = "秋 行 こ\nの く の\n暮 人 道\nれ な や\n\u{3000} し \u{3000}\n\u{3000} に \u{3000}\n------------\n";

        // Llamar a la función haiku_func y verificar el resultado
        let result = haiku_func(vec![input_haiku.to_string()], "output.txt", "output2.txt").await;
        assert!(result.is_ok()); // Verificar que la llamada a la función fue exitosa

        // Leer el contenido del archivo de salida
        let content = std::fs::read_to_string("output.txt").unwrap();

        // Verificar si el contenido coincide con el resultado esperado
        assert_eq!(content, expected_output);
    }
}
 */
