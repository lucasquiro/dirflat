use std::io;
use std::path::PathBuf;

pub fn ruta() -> PathBuf{
    
    // Se crea una variable mutable y vacia
    let mut input_menu = String::new();

    // Se recibe el valor ingresado por el usuario y se guarda en la variable input_menu
    io::stdin().read_line( &mut input_menu).expect("No se pudo leer el input del menu");

    /*se sanetiza el valor eliminando espacios al inicio las \ y el \n del final*/
    let menu_eleccion = input_menu.trim().trim_end().trim_matches('"').replace('\\', "");

    std::path::PathBuf::from(menu_eleccion)
}