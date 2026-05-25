use std::{fs};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;


fn main() {

    // Aqui lo que se hace es añadir las carpetas de origen y destino en las cuales vamos a hacer pruebas
    let origen: &Path= Path::new("/run/media/douner/2tb/prueba/origen");
    let destino: &Path= Path::new("/run/media/douner/2tb/prueba/destino");

    match verificador_de_rutas(origen, destino) {
        Ok(true) => {
            println!("Estamos en el mismo disco")
        }
        Ok(false) => {
            println!("{:?}, {:?}, Estan en discos distintos", origen, destino)
        }
        Err(error) => {
            println!("Ocurrio un error, {}", error)
        }
    }
}

fn verificador_de_rutas(origen: &Path, destino: &Path) -> Result<bool, String> {

    // Se comprueba si la ruta de origen y de destino son iguales
    if origen == destino {
        return Err(format!("la ruta de origen y destino son iguales"))
    }

    // Se comprueba si la ruta de destino esta dentro de la ruta de origen
    if destino.starts_with(origen) {
        return Err(format!("la ruta de destino esta dentro de la ruta de origen"))
    }

    // Aqui se verifica si la ruta de origen existe en caso de que no exista retorna un error
    if Path::new(origen).exists() == false {
        return Err(format!("la ruta {:?} no existe.", origen));
    }

    // Aqui se verifica si la ruta de destino existe en caso de que no exista retorna un error
    if Path::new(destino).exists() == false {
        return Err(format!("la ruta {:?} no existe.", destino))
    }

        // Aca se extraen los metadatos del disco de la ruta origen y luego se convierten a un u64 
        let metadatos_origen: fs::Metadata = fs::metadata(origen).unwrap();
        let id_origen: u64 = metadatos_origen.dev();

        // Aca se extraen los metadatos del disco de la ruta destino y luego se convierten a un u64 
        let metadatos_destino: fs::Metadata = fs::metadata(destino).unwrap();
        let id_destino: u64 = metadatos_destino.dev();

    // Aqui comparamos si ambas id son iguales {
    Ok(id_origen == id_destino)
}

fn contenido(origen: &Path, destino: &Path) {

    // Creamos un archivo vacio en el que vamos a escribir todo el arbol de directorios

    let mut arbol = File::create("arbol.txt").unwrap();



}