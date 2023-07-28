use std::collections::HashSet;
// use std::prelude::v1::*;
struct MyStruct {}

/*
    en cada modulo rust inyecta una directiva use por defecto
    en el prelude se incluyen elementos o funciones que se usan
    recurrentemente en rust. Y al estar definido en el prelude.
    y al inyectarse por defecto, no hay necesidad de usar un recurso
    que se encuentre en esa ruta.
 */
fn main(){
    //Prelude:: es 
    let my_struct: MyStruct;
    let hashSet: HashSet<i32>;
    let mapa: std::collections::HashMap<i32,i32>;
}