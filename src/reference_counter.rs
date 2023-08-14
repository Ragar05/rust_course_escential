use std::rc::Rc;

#[allow(unused_variables)]
fn main(){
    //Reference Counter Smart Pointer: permite que un valor tenga muchos duenos.
    //Se usa cuando se quiere asignar datos en el heap para que sea accedido en multiples
    //partes del codig, y no se puede determinar en tiempo de compilacion el ultimo
    //que accedera a estos datos. Si se supiera de antemano quien seria el ultimo, se
    //podria hacer que ese ultimo seal el dueno, pero al no saberlo. Entonces RC, lleva
    //un contador de referencias con todos los duenos, y cuando ya no quedan mas duenos
    //puede limiar el dato.

    enum List {
        Node(i32, Rc<List>),
        None,
    }
    use List::*;

    //QUE NODE1 Y NODE0 apunten a NODE2
    //node1 -> 
    //     node2 -> node3 -> none
    //node0 ->
    
    let node3 = Node(10, Rc::new(None));
    let node2 = Node(10, Rc::new(node3));
    let node2_rc =Rc::new(node2);
    println!("CONOCER EL NUMERO DE REFERENCIAS: {}", Rc::strong_count(&node2_rc));
    let node1 = Node(10, Rc::clone(&node2_rc)); //deep clone
    println!("CONOCER EL NUMERO DE REFERENCIAS: {}", Rc::strong_count(&node2_rc));
    let node0 = Node(5,Rc::clone(&node2_rc));
    println!("CONOCER EL NUMERO DE REFERENCIAS: {}", Rc::strong_count(&node2_rc));

}