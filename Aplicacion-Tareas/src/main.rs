use std::collections::HashMap;

fn main() {
    let accion = std::env::args().nth(1).expect("Por favor especifique una accion");

    let mut ejecutar = Ejecutar::nuevo().expect("Error al inicializar la base de datos");

    match accion.as_str() {
        "agregar" => {
            let articulo = std::env::args().nth(2).expect("Por favor especifique un articulo");
            ejecutar.insertar(articulo);
            match ejecutar.guardar() {
                Ok(_) => println!("Ejecucion exitosa y guardada"),
                Err(e) => println!("Ha ocurrido un error: {}", e),
            }
        },
        "completado" => {
            let articulo = std::env::args().nth(2).expect("Por favor especifique un articulo");
            match ejecutar.completado(&articulo) {
                None => println!("'{}' no se encuentra en la lista", articulo),
                Some(_) => match ejecutar.guardar() {
                    Ok(_) => println!("Ejecucion exitosa y guardada"),
                    Err(e) => println!("Ocurrio un error: {}", e),
                },
            }
        },
        "eliminar" => {
            let articulo = std::env::args().nth(2).expect("Por favor especifique un articulo");
            ejecutar.eliminar(&articulo);
            match ejecutar.guardar() {
                Ok(_) => println!("Tarea '{}' eliminada exitosamente", articulo),
                Err(e) => println!("Ocurrio un error al eliminar la tarea: {}", e),
            }
        },
        "listar" => {
            ejecutar.listar_tareas();
        },
        "editar_nombre" => {
            let viejo_nombre = std::env::args().nth(2).expect("Por favor especifique el nombre antiguo de la tarea");
            let nuevo_nombre = std::env::args().nth(3).expect("Por favor especifique el nuevo nombre de la tarea");
            ejecutar.editar_nombre(&viejo_nombre, &nuevo_nombre);
            match ejecutar.guardar() {
                Ok(_) => println!("Nombre de la tarea '{}' cambiado a '{}'", viejo_nombre, nuevo_nombre),
                Err(e) => println!("Ocurrio un error al editar el nombre de la tarea: {}", e),
            }
        },
        "editar_estado" => {
            let nombre_tarea = std::env::args().nth(2).expect("Por favor especifique el nombre de la tarea");
            ejecutar.editar_estado(&nombre_tarea);
            match ejecutar.guardar() {
                Ok(_) => println!("Estado de la tarea '{}' cambiado correctamente", nombre_tarea),
                Err(e) => println!("Ocurrio un error al editar el estado de la tarea: {}", e),
            }
        },
        _ => {
            println!("Comando no reconocido. Uso: cargo run [agregar|completado|eliminar|listar|editar_nombre|editar_estado] articulo [nuevo_nombre]");
        }
    }
}

struct Ejecutar {
    map: HashMap<String, bool>,
}

impl Ejecutar {
    fn nuevo() -> Result<Ejecutar, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("bdatos.json");

        match f {
            Ok(file) => {
                match serde_json::from_reader(file) {
                    Ok(map) => Ok(Ejecutar { map }),
                    Err(e) if e.is_eof() => Ok(Ejecutar { map: HashMap::new() }),
                    Err(e) => panic!("Error al leer el archivo: {}", e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn insertar(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn guardar(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("bdatos.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn completado(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = false;
                Some(())
            }
            None => None,
        }
    }

    fn eliminar(&mut self, key: &String) {
        self.map.remove(key);
    }

    fn listar_tareas(&self) {
        println!("Lista de tareas:");
        for (key, value) in &self.map {
            let estado = if *value { "Pendiente" } else { "Completada" };
            println!("{} - {}", key, estado);
        }
    }

    fn editar_nombre(&mut self, viejo_nombre: &str, nuevo_nombre: &str) {
        if let Some(_value) = self.map.remove(viejo_nombre) {
            self.map.insert(nuevo_nombre.to_string(), true);
        } else {
            println!("La tarea '{}' no existe", viejo_nombre);
        }
    }

    fn editar_estado(&mut self, nombre_tarea: &str) {
        match self.map.get_mut(nombre_tarea) {
            Some(estado) => {
                *estado = !*estado; //NOTE:Cambia el estado opuesto
            }
            None => {
                println!("La tarea '{}' no existe", nombre_tarea);
            }
        }
    }
}

//* En Rust todas las variables son inmutables por defecto
//TODO Explicacion:
//* 1. map: HashMap<String, bool> Es un HashMap que contiene una cadena y un booleano
//*Observacion:para que sea metodo se debe tomar self como primer argumento

//NOTE: Comandos:
// cargo run agregar "nombre de la tarea"
// cargo run completado "nombre de la tarea"
// cargo run eliminar "nombre de la tarea"
// cargo run listar
// cargo run editar_nombre "nombre de la tarea" "nuevo nombre de la tarea"
// cargo run editar_estado "nombre de la tarea"
