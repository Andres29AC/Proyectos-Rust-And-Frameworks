// Explicacion:
// 1. La macro #[launch] es una macro de Rocket que se encarga de iniciar el servidor.
// 2. La macro #[get] es una macro de Rocket que se encarga de definir una ruta.
// 3. La macro #[macro_use] 105105extern crate rocket; es una macro de Rust que se encarga de importar la libreria Rocket.
// &static str es un tipo de dato que se encarga de retornar una cadena de texto.
//
// Ciclo de vida:
// Primero se ejecuta las rutas
// Segundo se realiza un proceso de validacion
// Tercero se ejecuta el proceso de procesado
// Y por ultimo retorna la respuesta que se conoce como response
#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::Template;
use rocket::form::Form;
use rocket::serde::Serialize;
// use rocket::response::NamedFile;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
        Template::render("index", &context) 
}
#[derive(Debug, FromForm, Serialize)]
struct Accion<'r> {
    usuario: &'r str,
    password: &'r str,
    completed: bool
}

#[post("/", data = "<accion>")]
fn nuevo(accion: Form<Accion<'_>>) -> Template {
    if accion.usuario.is_empty()||accion.password.is_empty(){
        Template::render("index", &*accion)
    } else {
        Template::render("inicio", &*accion)
    }
}
#[get("/inicio")]
fn inicio() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    Template::render("inicio", &context)
}

// #[get("/fondo.jpg")]
// fn fondo_jpg() -> &'static str {
//      NamedFile::open("templates/fondo.jpg").map_err(|_| Status::NotFound)
// }
//let mut context: HashMap<&str, &str> = HashMap::new(); hace que se cree un objeto de tipo HashMap
//que es un tipo de dato que se encarga de almacenar datos en forma de clave y valor.


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, nuevo, inicio])
        .attach(Template::fairing())
}

