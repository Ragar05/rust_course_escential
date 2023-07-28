fn main() {
    let num: Option<i32> = Some(100);
    // let num: Option<i32> = None;
    match num {
        Some(value) => println!("NUMERO VALIDO {}", value),
        None => println!("Numero invalido")
    }

    // if let Some(value) = num {
    //     println!("NUMERO VALIDO {}", value)
    // } else {
    //     println!("Numero invalido")
    // }

    //Por defecto debe de terminar el programa con un panic()
    //Lanza un error que termina el programa
    let Some(_) = num else {
        panic!("Numero invalido")
    };
}