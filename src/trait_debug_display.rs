#[derive(Debug)]
struct User {
    nombre: String,
    edad: i32,
}

//El atributo Derive permite usar ciertas implementaciones de algunos traits
//Las macros nos permiten definir y modificar el comportamamiento de algo a gusto

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User {}, has {} ages.", self.nombre, self.edad)
//     }
// }

//Trait Display
struct UserAdmin {
    nombre: String,
    edad: i32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {\n nombre: {}\nedad: {}\n}", self.nombre, self.edad)
    }
}

fn main() {
    /*
       Macros: permite a los usuarios definir la extensión de sintaxis de forma
       declarativa. Rusta llama a tales extensiones "macros por ejemplo"
       o simplemente "macros".
    */
    let user = User {
        edad: 32,
        nombre: "Ramiro Andres Garcia Urueta".to_string(),
    };

    let user2 = UserAdmin {
        edad: 32,
        nombre: "Ramiro Andres Garcia Urueta".to_string(),
    };

    

    println!("usuario: {:?}", user);
    println!("usuarioAdmin: {:?}", user2);
}
