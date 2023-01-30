use regex::Regex;

fn main() {

    // Regex
    // (\d+)\s?\+\s?(\d+)
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_rest = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();




    // Traer datos del usuario
    println!("Por favor introduce tu expresión: ");  
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    println!("{:?}", expression); 
    
    // Multiplicación
    loop {
        // Aplicar Operaciones
        let caps = re_mult.captures(expression.as_str());

        if caps.is_none() {
            break
        }

        let caps = caps.unwrap();

        println!("{:?}", caps);
        
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mult = left_value * right_value;

        expression = expression.replace(cap_expression, &mult.to_string());
        println!("{:?}", expression); 
        
    }

    // Division
    loop {
        // Aplicar Operaciones
        let caps = re_div.captures(expression.as_str());

        if caps.is_none() {
            break
        }

        let caps = caps.unwrap();

        println!("{:?}", caps);
        
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mult = left_value / right_value;

        expression = expression.replace(cap_expression, &mult.to_string());
        println!("{:?}", expression); 
        
    }


    // Suma
    loop {
        // Aplicar Operaciones
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break
        }

        let caps = caps.unwrap();

        println!("{:?}", caps);
        
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
        println!("{:?}", expression); 
        
    }

    // Resta
    loop {
        // Aplicar Operaciones
        let caps = re_rest.captures(expression.as_str());

        if caps.is_none() {
            break
        }

        let caps = caps.unwrap();

        println!("{:?}", caps);
        
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let rest = left_value - right_value;

        expression = expression.replace(cap_expression, &rest.to_string());
        println!("{:?}", expression); 
        
    }
    // Mostrar resultado
    println!("Resultado {}", expression);
}