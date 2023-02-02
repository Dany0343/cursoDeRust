fn sumar_uno(numero_a_sumar: i16) -> i16 {  // Como en Python así se especifica el tipo que se va a devolver

    let numero_final = numero_a_sumar + 1;

    println!("{}", numero_final);

    return numero_final;
}



fn main() {
    println!("Holaaa");

    let diez_mas_uno: i16 = sumar_uno(10);
    sumar_uno(diez_mas_uno);
    sumar_uno(12);

}