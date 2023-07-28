fn main() {
    let edad: Option<i32> = Some(20);
    /*
    match edad {
        Some(_edad) => println!("Edad: {}", _edad),
        // None => ()
        _ => () //Cada caso que no se defina en el match, por defecto lo tomara como que no ejecutara algo
    }
    */

    //Podemos usar el if-let en comparacion al match
    if let Some(value) = edad {
        println!("Edad: {}", value)
    }

    let mut message_unreaded_counter = Some(100);

    // loop {
    //      match message_unreaded_counter {
    //         Some(value) => {
    //             if value > 0 {
    //                 println!("Tienes mensajes leidos: {}", value);
    //                 message_unreaded_counter = Some(value - 1)
    //             }else {
    //                 println!("No hay mensajes leidos");
    //                 message_unreaded_counter = None;
    //             }
    //         },
    //         None => { break;}
    //      }
    // }

    while let Some(value) = message_unreaded_counter {
        if value > 0 {
            println!("Tienes mensajes leidos: {}", value);
            message_unreaded_counter = Some(value - 1)
        } else {
            println!("No hay mensajes leidos");
            message_unreaded_counter = None;
        }
    };

}
