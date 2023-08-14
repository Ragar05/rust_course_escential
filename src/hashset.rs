use std::collections::HashSet;

fn main() {
    //Hashset -> garantiza que no hay elemento duplicados
    let mut user_ids: HashSet<i32> = HashSet::new();
    user_ids.insert(1);
    user_ids.insert(1);
    user_ids.insert(2);
    user_ids.remove(&1); //Los elementos a eliminar se pasan por referencia

    //METODOS de los hashset
    //union: obtener los elementos entre dos hashset
    //difference: obtener los elementos que estan en el primero set y no en el otro
    //intersection: obtener solo los elementos que estan en ambos sets
    //symetric_difference: obtener todos los elementos que estan en un set, o en el otro

    for id in user_ids.iter() {
        println!("{}", id)
    }
}
