/*
Prueba de tipo enumerado
*/

// Estructura tupla de pulsación de tecla
#[derive(Debug)]
struct KeyPress(String, char);

// Estructura clásica para la pulsación de ratón
#[derive(Debug)]
struct MouseClick { x: i64, y: i64 }

// Enumerado con tres variantes
// Para la variante WELoad se utiliza un booleano
#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

fn main() {
    // Instancia una variable de tipo pulsación de ratón
    // Con unas coordenadas concretas
    let click = MouseClick { x: 100, y: 250 };
    println!("Localización de la pulsación del ratón: {}, {}", click.x, click.y);
        
    // Instancia una pulsación del teclado
    let keys = KeyPress(String::from("Alt+"), 'P');
    println!("\nCombinación de teclas presionada: {}{}", keys.0, keys.1);
        
    // Instancia una variable de tipo WebEvent
    // En este caso es una de tipo WELoad
    let we_load = WebEvent::WELoad(true);

    // Instancia otra variable de tipo WebEvent
    // En este caso es una pulsación del ratón
    let we_click = WebEvent::WEClick(click);

    // Instancia otra variable de tipo WebEvent
    // En este caso es una pulsación del teclado
    let we_key = WebEvent::WEKeys(keys);
        
    // Imprime los valores de las variables
    // Se utiliza la sintaxis {:#?} para mostrar el contenido de estas estructuras
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
