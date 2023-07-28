#[derive(Debug)]
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }else {
            return  None;
        }
    }
}

fn main() {
    //Iteradores
    /*
        Un iterator es un Trait de la libreria estandar
        Los slices y los vectores lo implementan.
        Hay funciones de los iterators como sum, filter
        toman propiedad sobre los datos y se tienden a perder.
    */

    let s = [1, 2, 3];
    for x in s.iter() {
        println!("{}", x + 1)
    }

    println!("{:?}", s);

    let mut counter = Counter::new();
    counter.next();
    counter.next();
    counter.next();
    counter.next();
    counter.next();
    let index = counter.next();
    match index {
        Some(index) => println!("{:?}", index),
        None => println!("Llego al limite del contador"),
    };
}
