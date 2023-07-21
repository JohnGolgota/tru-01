// use std::collections::{HashSet, HashMap};

fn main(){
    println!("hola");
    // constantes_variables();
    // structuras_datos();
    estruct_poo();
}

// En verdad es como C pero C lo conozco entre nada y una 💩
fn estruct_poo(){
    // Estructura clasica por la clave valor... si...
    // Recuerda a una clase. aqui define los parametros
    struct Person { // Yo no nombro asi es la moricion
        nombre:String,
        edad:u32
    }

    // y de alguna forma los metodos
    impl Person {
        // Como un metodo constructor... sea que lo llamas cuando instancia la estructura...
        fn new(nombre: &str,edad: u32) -> Person {
            Person {
                nombre: String::from(nombre),
                edad
            }
        }
    }

    let persona_nueva=Person::new("cualquiera", 0);

    if persona_nueva.edad == 0 {
    println!("hola {}, que edad tienes? {}? es alguna broma? ", persona_nueva.nombre, persona_nueva.edad)
    } else {
        println!("hola {}, que edad tienes? {}?", persona_nueva.nombre, persona_nueva.edad)
    }
}