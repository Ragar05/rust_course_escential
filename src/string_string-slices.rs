fn main() {
    //String: se guarda en el heap ya que se desconoce su tamano, y este puede variar
    //String slide: se almacena en el stack. referencia al heap. string literals incrustados en el codigo binario

    let message: &str = "hola mundo"; //Se harcorde en el codigo final
    let message_v2: String = String::from("Hola mundo");
    
    println!("message: {}\nmessage_v2: {}", message, message_v2);

    let mut message2 = message.to_string(); //Se convierte &str a String
    message2.push('a');

    //String a String slide
    let message3 = &message_v2[..message_v2.len()];

}
