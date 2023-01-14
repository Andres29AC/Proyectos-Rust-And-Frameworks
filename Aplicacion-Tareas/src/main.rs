use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
fn main() {
    let accion = std::env::args().nth(1).expect("Por favor especifique una acción");
    let articulo = std::env::args().nth(2).expect("Por favor especifique un artículo");
    let mut ejecutar = Ejecutar{
        map: HashMap::new(),
    };
    let mut ejecutar = Ejecutar::nuevo().expect("Error al inicializar la base de datos");
    if accion == "agregar"{
        ejecutar.insertar(articulo);
        match ejecutar.guardar(){
            Ok(_) => println!("Ejecucion exitosa y guardada"),
            Err(e) => println!("Error: {}", e),
        }
    }else if accion == "completado"{
        match ejecutar.completado(&articulo){
            None => println!("'{}' no se encuentra en la lista", articulo),
            Some(_) => match ejecutar.guardar(){
                Ok(_) => println!("Ejecucion exitosa y guardada"),
                Err(e) => println!("Ocurrio un error!!: {}", e),
            },
        }
    };
    //println!("{:?}, {:?}", accion, articulo);
} 

//TODO Explicacion:
//* 1. let vincula un valor a una variable
//* 2. std::env::args() Es una función que devuelve los argumentos con los que se inicio el programa
//* 3. nth() Es un método de iterador que devuelve el elemento n-ésimo de un iterador
//* 4. expect() Sirve para manejar errores es un método de Result que devuelve el valor de un Result o se detiene con 
//*    el mensaje de error especificado

struct Ejecutar{
    map: HashMap<String, bool>,
}
impl Ejecutar{
    fn nuevo() -> Result<Ejecutar, std::io::Error>{
        let mut f =std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("base_de_datos.txt")?;
        let mut contenido = String::new();
        f.read_to_string(&mut contenido)?;
        let mut map = HashMap::new();
        
        for entradas in contenido.lines(){
            let mut valores = entradas.split('\t');
            let key = valores.next().expect("No se encontro la clave");
            let val = valores.next().expect("No se encontro el valor");
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        
        Ok(Ejecutar{map})
    }
    fn insertar(&mut self, key: String){
        self.map.insert(key, true);
    }
    fn guardar(self) -> Result<(), std::io::Error>{
        //* Se devuelve un Result que puede ser un error o un valor
        let mut contenido = String::new();
        for(k,v) in self.map{
            let registro = format!("{}\t {}\n",k,v);
            contenido.push_str(&registro);
        }
        std::fs::write("base_de_datos.txt", contenido)
    }
    //* Option es un tipo de dato que puede ser None o Some
    //* Option se utiliza para representar un valor que puedo o no existir
    fn completado(&mut self, key:&String) -> Option<()>{
        match self.map.get_mut(key){
            Some(v) => Some(*v = false),
            None => None,
        } 
    }
    
}

//* En Rust todas las variables son inmutables por defecto
//TODO Explicacion:
//* 1. map: HashMap<String, bool> Es un HashMap que contiene una cadena y un booleano
//*Observacion:para que sea metodo se debe tomar self como primer argumento
