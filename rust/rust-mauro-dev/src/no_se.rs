fn main(){
    println!("hola");
    basics()
}

fn basics() {
    // declaracion de una variable con el tipo y el valor
    // let hace referencia a una variable inmutable
    // let <nombre de variable>:<typo de dato> = <valor>
    let _cadena: &str = "cosas"; // la barra baja antes del nombre para que no este jodiendo por no usar las variables que declaro

    // formas de usar valores en cadenas de texto
    println!("invocado desde los parametros {}", _cadena);

    // Los tipos de datos pueden ser inferidos pero sigue siendo una variable inmutable
    let _cadena_dos = "otra cosa";

    println!("usado dentro de la cadena de texto {_cadena_dos}");
    // Con el primer tipo en principo deveria ser mas sencillo operar con las varibles.

    let _truth_cadena: String = String::from("Esta es una verdadera cadena de texto"); // ... sea lo que sea que eso signifique


    // Numeros hay tres graandes tipos
    // Enteros, enteros sin signo, y floats
    // y multiples tipos deribados de estos
    // desde 8 a 128 bits para los enteros y de 32 y 64 bits para los flotantes
    let _tupla = (9i32, 9u32, 9.0f32); 
    // De paso tupla tanbien es un tipo de dato

    // Rust aparte de ser un leguaje de tipado fuerte es estricto con la interaccion entre tipos
    // let suma_nums = _tupla.0 + _tupla.2; // este es un erro por que los tipos de estas variables no son el mismo

    // ocurre tambien en la misma familia de tipo me refiero a los 'u', 'i' o 'f' en los numeros
    // let resta_nums = 12i8 - 1i32; // aunque ambos sean de tipo entero con signo. sigen teniendo una asignacion de memoria distinta. diferentes bits... sabe'?
    

    // TODO mutabilidad

    // TODO shadow
}