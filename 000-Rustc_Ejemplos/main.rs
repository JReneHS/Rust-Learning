//fn significa que es una funcion
//Rust es un lenguaje compilado con rustc
fn main() {
    println!("Hola mundo"); // si algo tiene signo de admiracion es una macro
                            // Variables, constantes e inmutables
                            //Todas las variables son inmutables :vv
                            //si intentamos modificar la variable nos dara error
    let x: i32 = 1;
    //Para poder modificar su valor necesitas usar la palabra mut
    let mut y: i32 = 30;
    println!("x = {}", x);
    println!("y = {}", y);
    y = 15;
    println!("y es mutable >> y = {}", y);
    //Rust es capaz de inferir el tipo (auto automatico)
    let z = 1.15;

    const MI_PI: i32 = 1;

    println!("{}", z);
    println!("{}", MI_PI);
    //La diferencia entre una variable inmutable y una constante son
    //  *Las constantes siempre siempre requieren especificar el tipo
    //  *Las constantes se pueden declarar en el scope glabal
    //  Las constantes no pueden guardar funciones (como JS)

    //  Patern Matching
    //  let PATRON = EXPRESION
    //  let intenta destructurar la expresion y añadir los valores al patron

    //Tupla
    let zz: (i32, i32, i32) = (1, 2, 3);
    let (a, b, c) = (1, 3, "str");

    println!("La tupla zz tiene {:?}", zz);
    println!("a: {} b: {} c: {}", a, b, c);
}
/*
 * En Windows se crea un archivo con extencion .pdb que coniene informacion sobre debugging
 * Cargo es el sistema de construccion y manejador de paquetes de Rust (Parecido a node para JS)
 *
 * Con "Cargo new <Nomre-de-Proyecto>"
 * Cargo crea una carpeta diseñada para albergar un proyecto completo
 * dentro de la carpeta se crea:
 *  >> Cargo.toml (Tom's Obvious, Minimal Languaje)
 *  >> una carpeta src con un main.rs dentro
 *  >> un archiva .giticnore
 *
 * Dentro del archivo Cargo.toml contiene informacion del proyecto
 *  el encabezado [package] indica los estados o registros de configuracion del paquete
 *  Le sirve a Cargo para poder hacer una compilacion adecuada
 *
 *  el encabezado [dependencies] genera la lista de todas las dependencias del proyecto (crates)
 *
 * Cargo espera que todo tu codigo fuente este en la carpeta "src"
 * en la carpeta raiz o main solo debe haber:
 *  >> Cargo.toml
 *  >> Cargo.lock
 *  >> archivos README
 *  >> informacion de lisencias
 *  >> archivos de configuracion
 *  >> cualquier cosa que NOO este relacionada con el codigo
 *
 *
 * Para construir tu proyecto con cargo es tan sencillo como
 *  >>Situate en la carpeta Raiz de tu proyecto y ejecuta el comando:
 *      $ cargo build.
 * Lo cual genera un ejecutable en la carpeta
 *  target/debug/ <nombre-del-proyecto> (.exe en caso de windows)
 *
 * Al construir o compilar el proyecto se genera en la raiz el archivo Cargo.lock
 * Este archivo contiene el seguimiento exacto de todas las versiones de las dependencias
 * Cargo.lock esta diseñado para actualizarce solo. NO lo modifiques!!!!!
 *
 * Tambien se puede compilar y correr de manera automatica con
 *      cargo run
 *
 * con el comando "cargo check" revisa de manera rapida si es seguro compilarlo (PEROOO no produce
 * ningun ejecutable)
 *
 * cargo check es mucho mas rapido que cargo build
 *
 *
 * Cuando el proyecto esta listo para ser liberado Cargo tiene una funcion especial que se ejecuta:
 *      "cargo build --release"
 * El cual compila con optimizaciones para produccion
 * este comando crea una nueva carpeta dentro de target/debug
 * target/debug/release
 *
 * Estas optimizaciones hocen que corra mucho mas rapido el programa pero en consecuencia es muuuy
 * lenta la compilacion.
 *
 * cuando son proyectos sencillos o muy simples no hay diferencia tangible entre cargo y rustc
 * Pero al irse complicando los proyectos y al ir aumentando el numero de archivos, crates
 * (paquetes) y de clases es mucho mas facil dejar que cargo se encargue (7w7r) de la compilacion.
 *
 * */
