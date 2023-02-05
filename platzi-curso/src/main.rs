use csv::{ReaderBuilder, StringRecord};
// Esto permite leer un String que tenga forma de CSV
use std::{fs}; // Para poder traernos el archivo al código

// Traer hash map
use std::collections::{HashMap};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

// Función generadora de DatoHistoria
impl DatoHistoria {
    
    fn new(row: StringRecord) -> DatoHistoria {

        let vida = row.get(3).unwrap().trim();
        let vida = vida.parse().unwrap_or(0);

        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
            opciones: vec![]
        };
    }
}


fn main() {

    let mut vida = 100;
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();
    let mut tag_actual = FIRST_TAG; 
    let mut last_record: String = "".to_string();
    let content = fs::read_to_string(FILENAME).unwrap();

    // leer datos tabulares como lo es en un CSV
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
    // la 'b' significa que se pasarán los datos como bytes y no como strings

    for result in rdr.records() {

        let result = result.unwrap();
        let dato: DatoHistoria = DatoHistoria::new(result);

        if dato.tipo_dato == "SITUACION" {
            let record_tag = dato.tag.clone();

            datos_historia.insert(record_tag.clone(), dato);
            last_record = record_tag;
        }
        else if dato.tipo_dato == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record) {
                (*data).opciones.push(dato)
            }
        }
    }

    // Game Loop
    loop {
        println!("Tienes {} de vida", vida);

        if let Some(data) = datos_historia.get(tag_actual) {
            println!("{}", data.texto);

            for (indice, option) in data.opciones.iter().enumerate() {
                println!("[{}] {}",indice, option.texto);
            }

            // Traer datos del usuario
            let mut seleccion = String::new();
            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elegida) = &data.opciones.get(seleccion) {
                tag_actual = &opcion_elegida.tag;
            }
            else {
                println!("Comando no valido!");
            }

            vida += data.vida;
            println!("");
        }
        else {
            break;
        }

        // Si la vida es menor o igual a 0, entonces terminar juego
        if vida <= 0 {
            println!("Has perdido!");
            break;
        }
    }

}