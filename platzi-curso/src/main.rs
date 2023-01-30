fn main() {
    // Vectores
    let mut nombres: Vec<String> = Vec::new();
    
    println!("Por favor, introduce un nombre: ");
    let mut nombre = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();

    nombres.push(nombre);

    println!("{:?}", nombres); // Mostrar Vector de manera entera con el truco
}