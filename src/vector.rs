use std::fmt::Display;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    //let mut v = vec![1,2,3,4,5,6]; //Macro para crear macro rapidamente
    v.push(1);
    v.push(2);
    v.push(3);

    for value in v.iter() {
        println!("Elemento del vector: {}", value);
    }

    v.iter().for_each(|value| {
        println!("Elemento del vector: {}", value);
    });

    for value in &v {
        println!("Elemento del vector: {}", value);
    }

    // let element_indexed = v.get(100);

    // let Some(_) = element_indexed else {
    //     panic!("No existe el valor en el vector");
    // };

    //Acceder y modificar los valores
    for i in &mut v {
        //Como se accede a la referencia, se debe desreferenciar para modificar
        // i += 10;
        *i += 10;
    }

    for value in &v {
        println!("Elemento del vector modificado: {}", value);
    }

    #[derive(Debug)]
    enum Message {
        TEXT(String),
        ERROR(i32),
    }

    let messages = vec![
        Message::TEXT(String::from("holaaaa")),
        Message::TEXT(String::from("holaaaa")),
        Message::ERROR(32),
    ];

    for item in &messages {
        match item {
            Message::TEXT(value) => println!("Mensaje: {}", value),
            Message::ERROR(err) => println!("Error: {}", err),
        }
    }
}