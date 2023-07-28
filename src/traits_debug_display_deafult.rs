#[allow(dead_code)]
#[derive(Debug, Default)]
struct User {
    nombre: String,
    edad: i32,
}

//El atributo Derive permite usar ciertas implementaciones de algunos traits
//Las macros nos permiten definir y modificar el comportamamiento de algo a gusto

//Trait Debug
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User {}, has {} ages.", self.nombre, self.edad)
//     }
// }

//Trait Display
//Con el trait display yo defino como quiero que mi struct se imprima en consola
//cuando se pasa por parametros
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "User: {{\n nombre: {}\nedad: {}\n}}", self.nombre, self.edad)
        write!(f, "{} - ({})",self.nombre, self.edad)
    }
}

//Trait Default
//Permite crear estructuras y definir enums con valores por defecto
//Los enum solo permiten default value en enums unitarios, osea que no tengan parametros
#[allow(dead_code)]
#[derive(Debug, Default)]
enum Courses {
    #[default]
    NONE,
    JARDIN,
    TRANSICION,
    PRIMERO,
    SEGUNDO,
    TERCERO,
    CUARTO,
    QUINTO
}

#[derive(Debug, Default)]
struct  Student {
    name: String,
    course: Courses
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
    
    //USE TRAIT DISPLAY
    println!("usuario: {}", user);
    //USE TRAIT DEBUG
    println!("usuarioAdmin: {:?}", user);


    //USE TRAIT DEFAULT
    let mut student = Student::default();
    println!("Estudiante por defecto: {:?}", student);
    student.course = Courses::PRIMERO;
    student.name = "Ramiro Andres Garcia Urueta".to_string();
    println!("Estudiante por modificado: {:?}", student);

}
