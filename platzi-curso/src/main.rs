fn main() {
    // Dos numeros que se van a sumar
    let num_1 = 120; // Como es obvio que la variable tiene un valor numerico, no es necesario ponerle el tipo, Rust automaticamente lo asigna.
    let num_2 = 321;

    let suma = num_1 + num_2;

    loop {
        // Mostrar los dos números en pantalla
        println!("Por favor escribir la suma de {} y {}:", num_1, num_2);

        // Obtener del usuario el número que representa la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int: i16 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has hecho muy bien, el resultado es correcto");
            break;
        }
        else {
            println!("El resultado {} no es correcto, por favor intentalo de nuevo", suma_usuario_int);
        }

    }
}