fn main() {
    println!("Por favor, introduce tu nombre: ");

    let mut nombre: String = String::new();

    // Capturar datos del usuario
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Un ejemplo un poco más complicado
    // Obtener la edad de la consola
    let mut pais: String = String::new();
    println!("Por favor, introduce el país de donde vienes: ");
    std::io::stdin().read_line(&mut pais).unwrap();
    pais = pais.trim().to_string();

    println!("Hola, bienvenidx {} del país {}", nombre, pais);
}
