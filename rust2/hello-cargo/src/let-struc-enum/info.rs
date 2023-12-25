fn hola(nombre: &str) {
    println!("hola {}", nombre);
}

fn numeros_texto_var() {
    // Números enteros
    // Números de punto flotante
    // Valores booleanos
    // Characters

    // Uso de variables en fn
    let number: u32 = 14;
    println!("The number is {}.", number);

    let numer_i: i8 = -1;
    println!("The number is {}.", numer_i);

    // Numeros Mal
    // let number: u32 = "14";

    // Tipos de numeros enteros y flotantes
    let typos_numeros_firmado = (2i8, 16i16, 8i32, 64i64, 128i128, 1isize); // estos numeros tienen signo => pueden ser negativos y su valor maximo es reducido a la mitad en comparacion a los u
    let typos_numeros_con_signo = (9u8, 2u16, 32u32, 64u64, 128u128, 1usize); // no se a que se refiere firmado Contexto? => no peden ser negativos
                                                                              // flotantes
    let number_64 = 9.0; // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        typos_numeros_firmado.2 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!(
        "9 / 2 = {} but 9.0 / 2.0 = {} y {}",
        typos_numeros_con_signo.0 / 2,
        number_64 / 2.0,
        number_32
    );
}

fn boleanos() {
    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);
}

fn texto() {
    // Caracteres y cadenas
    // En rust existe el tipo char especifico para un caracter unicode o algo así

    // Char un solo caracter y asignado usando comillas simples
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';

    println!(
        "Caracteres ijuep... {} {} {}",
        uppercase_s, lowercase_f, smiley_face
    );

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃'; // what no se supone que era inmutable? y mi error? => ya lo entendi es un shadow. esta prevalece y la anterior ya no se puede hacer referencia

    // Strings
    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );
}

fn mutabilidad() {
    // Por defecto todas las variables en rust son inmutables
    // error: intentar reasignar el valor de la variable
    // let a = 2;
    // a = 3;

    // Error: reasignar el valor de una variable mutable sin usar el valor anterior
    // let mut a = 2
    // a = 3;

    // Error asignar una unica vez el valor de una variable mutable
    // let mut a;
    // a = 3;

    // Bien: usarla
    let mut a = 2;
    println!("num {}", a);
    a = 3;
    println!("num {}", a);
}

fn shadow() {
    let shadow_num = 2; // esta variable no es sobre escrita

    // Creamos otra variable con el mismo nombre. y la anterior pierde esa asignacion.
    // No se entonces que pasa con el valor que contenia.
    let shadow_num = shadow_num + 1; // esta variable se pone por delante de la anterior que comparte el nombre
    println!("sombra the shadow {}", shadow_num);

    // En mi cabeza ocure esto: var = 'algo' => var = 'otro algo' dejando lo anterior como: '' => 'algo'
}

fn tuplas_f() {
    // Longitud fija despues de ser declarada
    // los elementos ni se añaden ni quitan
    // SEA no es un array de js

    let cosas = ("cosa uno", 2, 3.0, true, 'S');

    println!(
        "para acceder a tuplas {}, {} se puede {}",
        cosas.0, cosas.4, cosas.3
    )

    // Muy parecido a un array inmutable
}

fn struct_unitaria() {
    let evento_vacio = Unit;

    hacer_algo_especial(evento_vacio);

    struct Unit; // Definimos la struct unitaria

    // Implementamos un trait vacío para la struct unitaria
    trait MarcarComoEspecial {}

    impl MarcarComoEspecial for Unit {}

    // Función que utiliza el trait como marcador
    fn hacer_algo_especial<T: MarcarComoEspecial>(_valor: T) {
        println!("¡Algo especial sucedió!");
    }
}

fn estructuras() {
    // Una de las caracteristicas principales de un struct es que puedes poner nombres. Objeto de toda la vida
    // Hay tres diferentes struct's que vamos a ver aquí
    // Las estructuras usan mayusculas

    // Classic struct with named fields
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    // struct Unit; // no se

    /*
    Standard library
    Global
    The global memory allocator, Global, is a unit struct:

    pub struct Global;
    It has no state of its own (because the state is global), but it implements traits like Allocator.

    std::fmt::Error
    The error for string formatting, std::fmt::Error, is a unit struct:

    pub struct Error;
    It has no state of its own, but it implements traits like Error.

    RangeFull
    The type for the .. operator, RangeFull, is a unit struct:

    pub struct RangeFull;
    It has no state of its own, but it implements traits like RangeBounds.

    Crates
    chrono::Utc
    The Utc timezone is a unit struct:

    pub struct Utc;
    It has no state of its own, but it implements traits like TimeZone and is thus usable as a generic argument to Date and DateTime.

    from stackoverflow https://stackoverflow.com/questions/67689613/what-is-a-real-world-example-of-using-a-unit-struct
    */

    // Instancia de una estructura
    let uno = Student {
        name: String::from("Ni puta idea"),
        level: 5,
        remote: false,
    };

    println!(
        "estudiante Estrella {} grado {}:{}",
        uno.name, uno.level, uno.remote
    );

    // Mas instancias
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false,
    };

    println!(
        "{}, level {}. Remote: {}.",
        user_2.name, user_2.level, user_2.remote
    );

    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student {
        name: String::from("Constance Sharma"),
        remote: true,
        level: 2,
    };
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false,
    };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );
}

fn enum_dotos_compuestos() {
    // enum WebEvent {
    //     // An enum variant can be like a unit struct without fields or data types
    //     WELoad,
    //     // An enum variant can be like a tuple struct with data types but no named fields
    //     WEKeys(String, char),
    //     // An enum variant can be like a classic struct with named fields and their data types
    //     WEClick { x: i64, y: i64 },
    // }

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    let mouse_click=MouseClick{ x:12, y:13};
    let keys_new_file=KeyPress(String::from("Ctrl+"),'N');
    println!("Vive y deja vivir x {} & y {}",mouse_click.x, mouse_click.y);

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    let e_num = WebEvent::WELoad(true);
    let w_click = WebEvent::WEClick(mouse_click);
    let w_keys = WebEvent::WEKeys(keys_new_file);

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", e_num, w_click, w_keys);

}