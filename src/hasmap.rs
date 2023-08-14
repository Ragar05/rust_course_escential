use std::collections::HashMap;
#[allow(dead_code, unreachable_code, unused_variables)]
fn main() {
    //HashMapo -> alamacen clave valor

    let mut map : HashMap<i32,String> = HashMap::new();
    map.insert(1, "SUPERADMIN".to_string());
    map.insert(2, "ADMIN".to_string());
    map.insert(3, "SUPERUSUARIO".to_string());
    map.insert(4, "USUARIO".to_string());

    let super_admin_id = map.get(&1);

    map.entry(5).or_insert("ASDADASD".to_string()); //SI no encuentr al key 5 por defecto a este key le dara el valor de que se definiio
    for (key,value) in map {
        println!("key: {}, value: {}", key,value)
    }

}
