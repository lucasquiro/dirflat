use crate::ui::ruta;
use crate::directorio::verificador_de_rutas;
mod ui;
mod directorio;

fn main() {

    println!("Escribe la ruta de origen");
    let origen:std::path::PathBuf = ruta();
    
    println!("Escribe la ruta de destino");
    let destino:std::path::PathBuf= ruta();

    match verificador_de_rutas(&origen, &destino) {
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