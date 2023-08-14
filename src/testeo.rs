fn main(){
    //testear
   //assert = asegurar
   //assert(expression) => expression is true
   //assert_eq!(a,b) => a is equal to b
   //assert_ne!(a,b) => a is not equal to b
}

fn sumar(a: i32, b: i32) -> i32 {
    return a + b;
}

fn is_only_numbers(codigo: &str) -> bool {
    return codigo.chars().all(char::is_numeric);
}

#[test]
fn sumar_bien(){
    assert_eq!(sumar(1, 3), 4);
}

#[test]
fn sumar_mal(){
    assert_ne!(sumar(1, 3), 5);
}

#[test]
fn codigo_con_numeros(){
    let result = is_only_numbers("12312334");
    assert!(result)
}

#[test]
fn codigo_con_letras(){
    let result = is_only_numbers("12312334a");
    assert!(!result, "El codigo contiene letras y lo valido como correcto")
}