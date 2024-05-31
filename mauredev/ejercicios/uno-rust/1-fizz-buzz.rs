/*
 * Escribe un programa que muestre por consola (con un print) los
 * números de 1 a 100 (ambos incluidos y con un salto de línea entre
 * cada impresión), sustituyendo los siguientes:
 * - Múltiplos de 3 por la palabra "fizz".
 * - Múltiplos de 5 por la palabra "buzz".
 * - Múltiplos de 3 y de 5 a la vez por la palabra "fizzbuzz".
 */

fn fizzbuzz(n: i32) -> String {
    let input = n.to_string();
    let s = match n % 15 {
        n if n % 3 == 0 && n % 5 == 0 => "fizzbuzz",
        n if n % 3 == 0 => "fizz",
        n if n % 5 == 0 => "buzz",
        _ => input.as_str(),
    };
    s.to_string()
}

fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}
