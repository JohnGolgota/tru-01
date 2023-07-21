
fn not_main() {
    // hola("Yo mero")
    // tuplas_f();
    // estructuras();
    // enum_dotos_compuestos();

    let nom = "Yo";

    let num = 3;
    println!("hola {}\n", nom);
    goodbye(nom);

    println!("suma fucked 2 + 2 = 22 || {:#?}", suma(2, 2));

    println!("division {} / 2 = {}", num, division(num));
}

fn goodbye(n:&str) {
    println!("adios. {}",n);
}

// Funcion con return
// define el tipo de dato retornado antes de entrar al bloque de codigo
// palabra clave return obligatoria
fn suma(num:u32, num_2:u32) -> u32 {
    return num + num_2;
}

fn division(num:i32) -> i32 {
    // bloque if
    if num == 0 {
        return 0;
    }
    return num / 2;
}

fn cosas() {
    #[derive(Debug)]
    struct Carrito {
        color: String,
        transmision: Transmision,
        convertible: bool,
        kilometrage: u32
    }

    // No se que es el PartialEq
    #[derive(PartialEq, Debug)]
    enum Transmision {
        Manual,
        SemiAuto,
        Automatico
    }

    fn car_factory(color: String, transmision: Transmision, convertible: bool) -> Carrito {
        let car = Carrito {color: String::from(color), transmision: transmision, convertible: convertible, kilometrage: 0};
        return car;
    }

    let mut car = car_factory(String::from("Cyan"), Transmision::Manual, false);
    println!("Car 1 = {}, {:?} transmision, convertible: {}, kilometrage: {}", car.color, car.transmision, car.convertible, car.kilometrage);

    car = car_factory(String::from("Plateado"), Transmision::Automatico, true);
    println!("Car 2 = {}, {:?} transmision, convertible: {}, kilometrage: {}", car.color, car.transmision, car.convertible, car.kilometrage);

    car = car_factory(String::from("Marron"), Transmision::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmision, convertible: {}, kilometrage: {}", car.color, car.transmision, car.convertible, car.kilometrage);    
}