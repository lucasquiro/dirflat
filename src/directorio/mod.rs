use std::{fs};
use std::path::Path;
use std::os::unix::fs::MetadataExt;

pub fn verificador_de_rutas(origen: &Path, destino: &Path) -> Result<bool, String> {

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