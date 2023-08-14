use std::ops::Deref;

#[allow(dead_code, unreachable_code, unused_variables)]
fn box_explanation() {
    //referencias: (&): apunta a una direccion de memoria
    let x = 5;
    let y = &x; //Y apunta a la direccion de memoria de x;

    /* Los smartpointer: son estructuras de datos que al igual que
    las referencias apuntan a una direccion de memoria. Ademas, trae
    informacion adicional (metadata).
        Reference counter: contador de referencias. permite a diversos
        dueños acceder al mismo dato. Se encarga de limpiar la data cuando
        ya no quedan dueños.

        Ejemplo:
            String
            Vec<T>

    Los smartpointers son implementeados usando structs pero implementado.
    los traits defer y drop.

    Deref: permite a las instancias de smartpointer comportarse como referencias
    para que el mismo codigo que funciona con referencias,funcione con smart pointers.

    Drop trait: permite definir logica que se ejecuta una vez que el smartpointer sale
    del scope.
    */

    //standar library

    // Box<T> permite almacenar datos en el heap
    let x = 2; //Se almacena el stack
    let y = Box::new(2); //Se almacena en el heap
                         /* Le permite a rust saber cuanta memoria tiene que asignar a las variables
                         que van a ser almacenadas en el heap. Cuando hay una variable que se desconoce
                         cuanta memroria va a utilizar. Es uitl definir dentro de un box,
                         ya que no necesita saber  cuanta memoria va utiliza el valor declarado.
                         Solo necesita saber cuanto va a ocupar la variable en stack, para
                         luego ampliarla en el heap.
                         */

    //Ejemplo linkedlist
    //(value1, node1) -> (value2, node2) -> (value3, node3) -> (value4, None);

    //Con estruturas de datos recursivas, rust no sabe como asignar el
    //tamano de la memoria. por lo tanto es necesario declara el valor tipo Box
    //PERMITE DELIMITAR LA MEMORIA NECESARIA PARA GUARDAR LO QUE ESTA EN EL ENUM
    enum List {
        Node(i32, Box<List>),
        None,
    }
    use List::*;

    let node3 = Node(10, Box::new(None));
    let node2 = Node(10, Box::new(node3));
    let node1 = Node(10, Box::new(node2));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!(
            "Me ejecuto ya que el smartpointer\n
        de y ya salio del scope"
        )
    }
}

#[allow(dead_code, unreachable_code, unused_variables)]
fn deref_and_drop_explanation() {
    //Deref trait: hace posibel la desrefrenciacion (*)

    let x = 5;
    let y = &x;
    let z = &y;
    let a = Box::new(x);

    if x == 5 {
        println!("hola 5");
    }

    /*
       Para realizar esta comparacion. Es necesario desreferenciar
       y. Ya que y no es un valor real, es una referencia que
       apunta a la posicion de memoria del dato almacenado en
       la variable x.
       por lo tanto y == 5 dara error.
       *y le dice a rust que tome el valor real. la cantidad de *
       depende de cuantas veces haya sido referenciada la variable.
       El smartpointer Box, ya implementa el trait deref.

       como forzar un drop
       let y = MyBox::new(23); 
       drop(y); //Se forza el drop e imprime el mensaje de cuando se realiza drop
    */
    if *y == 5 {
        println!("hola 5");
    }

    if **z == 5 {
        println!("hola 5");
    }

    if *a == 5 {
        println!("hola 5");
    }

    let x2 = 5;
    let y2 = MyBox::new(x);
    let z2 = MyBox::new(23);
    if *y == 5 {
        println!("hOLA DENTRO DEL IF");
    }
 
    drop(z2); //Se imprime el mensaje del trait drop implementado
    println!("EJECUCION DESPUES DEL DROP DE la variable z2");
}

#[allow(dead_code, unreachable_code, unused_variables)]
fn main() {
    box_explanation();
    deref_and_drop_explanation();
}
