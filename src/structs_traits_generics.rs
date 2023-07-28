//Genericos y (traits = rasgo)

struct Point<T> {
    x: T,
    y: T,
}

fn add_points(point_a: Point<f32>, point_b: Point<f32>) -> f32 {
    return point_a.x + point_a.x + point_b.y + point_b.y;
}

/*
    Traits: definen el comportamiento de un tipo y este puede
    compartirlo con otros tipos, ayuda a definir el comportamiento
    de manera abstracta entre varios tipos. Funciona como las
    interfaces en los lenguages como java y c#.
*/
struct Human;
struct Cat;
trait Talk {
    fn say_hello(&self) -> String;
    fn language() -> String {
        "No tengo idioma".to_string()
    }
}

impl Talk for Human {
    fn say_hello(&self) -> String {
        "Hola".to_string()
    }
    fn language() -> String {
        "Hablo humano".to_string()
    }
}

impl Talk for Cat {
    fn say_hello(&self) -> String {
        "Maow".to_string()
    }
}

//Definir que tipo de genericos recibira la funcion
fn say_hello_for_any<T: Talk>(any_to_talk: &T) {
    println!("{}", any_to_talk.say_hello());
}


trait LicenciaConducir {
    fn is_adult(&self) -> bool;
}

//Los traits permiten agregarle funcionalidades personalizadas al lenguage
// Como por ejemplo el siguiente ejemplo
impl LicenciaConducir for Option<i32> {
    fn is_adult(&self) -> bool {
        match self {
            Some(edad) => edad > &18,
            None => false
        }
    }
}
fn main() {
    let point_a = Point { x: 5, y: 9 };

    let point_b = Point { x: 4, y: 9 };

    println!(
        "
    point_a:
      x: {}
      y: {}
    point_b:
      x: {}
      y: {}
    ",
    point_a.x,
    point_a.y,
    point_b.x,
    point_b.y,
    );

    let user = Human;
    let animal = Cat;

    say_hello_for_any(&user);
    say_hello_for_any(&animal);

    let edad = Option::Some(19);
    println!("Es mayor de edad: {}", edad.is_adult())

}
