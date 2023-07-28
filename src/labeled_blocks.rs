fn main() {
    //Labeled Blocks

    let numero: Option<i32> = Some(101);

    let result:i32 = 'procedure_one: {
        'first_loop: loop {
            let Some(value) = numero else {
                break 'procedure_one 0; //Break o salir del bloque comentado
            }; //Si es None devolvera 0

            if value > 100 {
                break 'first_loop 100;
                //Si value es mayor que 100, devuelve 100
            }else{
                break 'procedure_one value;
                //Si value es es menoor que 100 devolvera value
            }
        }
    };

    println!("El resultado es: {}", result)
}
