fn structuras_datos(){
    // Estructura vec de vectores... ni idea.
    let mut una_estructura: Vec<&str> = vec!["primero", "segundo", "tercero"];
    println!("severa estructura {:?}", una_estructura);

    // una_estructura.push(1); // espera el mismo tipo de dato para el vector
    let cuatro:&str = "cuatro";
    una_estructura.push(cuatro);
    println!("nuevo elemento {:?}", una_estructura);
    // un Vec es una estructura de datos tipo array que almacena multiples valores del mismo tipo.
    // aparte de ser general mente inmutable

    // Sets es una estructura inportada.
    // Coloeccion de datos del mismo tipo
    let mut un_set: HashSet<u8> = [1u8,2u8,3u8,4u8].into();
    un_set.insert(1u8); // esta instruccion le valio verga
    // los HashSet no repiten valores dentro de la coleccion
    un_set.insert(5u8); // esto si lo hace
    println!("otra estructura {:?}", un_set);

    // Maps estructura inportada
    let mut un_mapa: HashMap<&str,&str> = HashMap::from([("esto", "me voy a matar"),("no se", "si se")]);
    println!("{:?}", un_mapa);
    un_mapa.insert("dios","no puedo mas");
    println!("{}, quiero dormir", un_mapa["dios"]);

    // estos pinc for mutan las estructuras. muy xd
    // El & para que haga referencia o apunte a la estructura sin mutarla o desplazarla no se que hace ;-;
    for valor in &una_estructura { // foreach por defecto... creo
        // for valor in una_estructura {
        println!("El {}", valor);
    }

    for valor in &un_set {
    // for valor in un_set {
        println!("\n{}", valor);
    }

    for (clave, valor) in &un_mapa {
    // for (clave, valor) in un_mapa {
        println!("\nVale verg {} por que {}", clave, valor);
    }

    if una_estructura.len() > 0 {
        println!("{:?}", una_estructura);
        println!("{:?}", un_set);
        println!("{:?}", un_mapa);
    }
    let mut contador = 0;
    while contador < 2 { // existe el .legth para hacer que esto sea un foreach
        println!("aaaaaaaaaaa");
        contador = contador+1; // seria gracioso comentar esto
        // Lo hice y respresent lo que siento ahora mismo.
    }
}