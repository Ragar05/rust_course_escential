use std::fs::File;
use std::io::ErrorKind;

fn main(){
    //Manejo de errores;
    //Recuperables: ej: abrir un archivo donde el path es incorrecto.
    //Result<T,E> => E type error
    
    //No-recuperables: ej: trata de accedeer a un arreglo mas alla de su limite
    //panic!("explosion") //forzar el error
    //permite ubicar en donde ocurrio el error

    let file = File::open("algun/path/a.rc");
    
    match file {
        Ok(_) => println!("existe el archivo"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("el archivo no fue encontrado"),
            _other_error => println!("Error desconocido")
        }
    }

    //unwrap => si todo esta bien retorna el OK. SI FALLA HACE UN PANIC
    // let file2 =File::open("algun/path/a.rc").unwrap();
    // SI FALLA RETONAR EJECUTA UN PANIC CON EL ERROR ESPERADO
    let file3 =File::open("algun/path/a.rc").expect("EL ARCHIVO NO SE ENCONTRO");

}

fn readFile(file:File){

}