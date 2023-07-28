fn main() {
    //Clousure: funcion que es definida en linea (inline).
    // recibe muchos parametros
    // | = pipe
    /* Para manuplar una variable dentro de un clouseure, 
       se debe colocar tanto la varibale como el clousere
       como mutable */

    let sum = |nro1: i32, nro2: i32| nro1 + nro2;

    let mut counter = 1;
    //Esta funcion toma los valores como copia y no como referencia
    let mut increment_two = move || {
        counter += 1;
    };
    let mut increment = || counter += 1;

    increment();
    let borrowing_counter = &counter;
    increment_two();
    println!("suma(4+6): {}", sum(4, 2));
    println!("counter: {}", borrowing_counter);
}
