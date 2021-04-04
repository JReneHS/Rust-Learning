/*
 * El juego consiste en generar numeros aleatorios entre 1 y 100
 * El jugador introduce un numero y el programa indicara si es bajo o alto su conjetura
 * si su conjetura o suposicion es correcta el programa imprime un mensaje de felicitaciones
 * */

//Libreria basica estandar (Sera igual que la std de C++ <emoji de duda>)
use rand::Rng;
use std::cmp::Ordering;
use std::io;

//Libreria para funciones de entrada y salida (input/output)
//Libreria que viene del crate rand (Random number generators)
//libreria para comparaciones

//Por default Rust brinda algunos tipos dentro del scope en el "Preludio"
//Si las funciones o macros no estan en el preludio es necesario indicarlas con "use"

fn main() {
    println!("El juego de las Conjeturas!!! (Aplausos)");

    let secret_number = rand::thread_rng().gen_range(1, 101); // Funcion que genera numeros aleatorios

    // println!("El numero Secreto es: {}", secret_number);

    // La funcion rand::thread_rng() da un tipo particular de funcion generadora:
    //  La funcion que retorna es local al hilo de ejecucion y es sembrado (seeded) por el O.S.
    //  y el sistema operativo se encarga de generar la semilla

    // El metodo gen_range() esta definida por las caracteristicas de Rng dentro del scope de la
    // instuccion dada en "use rand::Rng"
    // Este metodo toma dos numeros como arumentos y genera un numero random que este entre ellos.
    // Es inclusivo solo con el nivel inferior, es decir [x:y)
    // Inclusivo en Lower Boud pero exclusivo en Upper Bound.

    loop {
        println!("Por favor ingrese su suposicion");

        let mut guess = String::new(); //variable mutable | constructor de una variablo tipo string

        //Esta linea crea (declara) una variable mutable e instancia una string vacia
        //Las strings por defecto manejan el coder UTF-8
        //En este contexto :: actua como una funcion de asociacion
        //Una funcion de asociacion es implementada en un tipo (String en este caso)
        //En otros lenguajes se puede llamar metodo estatico

        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer la linea");

        // Si no se declara al principio "let std::io" se tendria que escribir toda la ruta
        //  std::io::stdin()...

        //  .read_line(&mut guess) es el llamado a un metodo del manejador de la entrada estandar
        //  Lo que hace es tomar lo que sea de la entrada estandar y meterlo a una string (el argumento)
        //  El argumento debe de ser si o si mutable
        //  & indica que es una referencia (como en casi todo lenguaje que se respeta)
        //  Las referencias son inmutables a priori, por eso es necesario indicar &mut para modificar.

        // Cuando llamas a un metodo con la sintaxis .foo() (.metodo()) es aconsejable separar en varias
        // lineas cuando el metodo es muy largo para facilitar la lectura

        // .expect("Error al leer la linea"); La funcion anterior (.read_line()) retorna un valor tipo
        // io::Result como indicador. Rust tiene un gran numero de tipos llamados "Result" en la libreria
        // estandar (std)
        // los tipos "Result" son enumeraciones, es decir hacen referencia a enums.
        // Existen muchos variantes de enums
        // entre ellos pueden tomar valor de Err o de Ok dependiendo si la operacion fue exitosa o no.
        // la variante Err cantiene informacion de porque fallo la operacion.

        // Rust no te obliga a usar las funciones expect() pero te mostrara Warnings al momento de
        // compilacion ya que la prioridad de Rust es generar codigo robusto que sea resistente a fallos.

        println!("Tu conjetura es: {}", guess); //los brakets {} son lugares para poner valores

        // Hacen referencia a pequeñas pincitas de cangrejo que sostienen los valores en su lugar. Kawai
        // Para generar numeros random haremos usa de un "crate"
        // los crate son colecciones de codigos fuente de Rust
        // Este proyecto como tal es un crate binario, que es un ejecutable
        // el crate "rand" es una libreria crate que contiene codigo diseñado para ser usado en otros
        // programas.

        // Todos los crate se tienen que descargar de internet
        // Para agregarlos al proyecto se tiene que modificar el archivo Cargo.toml
        // debajo de la seccion de [dependencies] se agrega la linea para este caso
        //      rand = "0.5.5"
        // Tambien se puede poner "^0.5.5" para instalar la version mas alta a partir de la "0.5.5" pero
        // que este por debajo de la  "0.6.0"

        // En el archivo Cargo.lock se guarda la version de los crate cuando corriste por primera vez
        // cargo build para evitar que en futuras actualizaciones ingresen bugs

        // Si uno corre el comando "cargo update" y actualiza todos los crate a su version mas acual.
        //e ignora la version indicada en cargo.lock.
        //Estas actualizaciones siempre seran en el margen menor, es decir que rondaran entre
        //  "0.5.5" y "0.6.0" (no inclusivo)
        // Asi que si quieres una version "0.6.0" o "0.6.x" tienes que modificar el Cargo.toml y
        // volver a correr el comando cargo build.

        // Rust tiene un sistema Fuertemente Tipado, lo que significa que no hoce castings de forma
        // discreta, entonces en este caso como estamos manejando una string, no podemos compararla con
        // un i32 (int de 32 bits)

        // Por ese Motivo tenemos que hacer la conversion... en este caso se modificara la funcion
        // para que lea un u32 (unsigned int de 32 bit)

        // shadow de guess para evitar dobles variables
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ingresa un valor numerico... Gilipollas!!\n");
                continue;
            }
        };

        // Trim Remueve cualquier espacio o caracter vacio (\n) tando prefix como sufix.
        // Parse busca cualquier tipo de numero y hace la conversion (esste caso u32)
        // En este caso se usa un match en vez del catch de exepciones "expect()"
        // Pasamos de fallar en un error a TRATAR y MANEJAR un error
        // Si el análisis puede convertir correctamente la cadena en un número,
        // devolverá un valor Ok que contiene el número resultante y lo colocará dentro
        // del valor de Ok. Ese número terminará justo donde lo queremos
        // en la nueva variable de conjetura que estamos creando.

        // Si el análisis no puede convertir la cadena en un número,
        // devolverá un valor Err que contiene más información sobre el error.
        // El guión bajo, _, es un valor general; en este ejemplo, estamos diciendo que queremos
        // hacer coincidir todos los valores de Err, sin importar la información que contengan.
        // Entonces, el programa ejecutará el código del segundo brazo
        // ¡el programa ignora todos los errores que puede encontrar el análisis!

        // Rust nos permite sombrear el valor anterior de la conjetura con uno nuevo.
        // Esta función se utiliza a menudo en situaciones en las que desea convertir un valor de un tipo a otro.
        // El sombreado nos permite reutilizar el nombre de la variable de conjetura en lugar de
        // obligarnos a crear dos variables únicas, como guess_str y guess, por ejemplo.

        // Al igual que Result Ordering es un enum, por la tanto tiene valores asociados, en este caso:
        //      Less
        //      Greater
        //      Equal

        // Son tres valores de retorno cuando haces comparaciones.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy Bajo"),
            Ordering::Greater => println!("Muy Alto"),
            Ordering::Equal => {
                println!("Ganaste!!!!");
                break;
            }
        }

        // Un match se maneja por brazos (arms x => y) que esta formado por un patron seguido de las
        // acciones a ejecutar en caso de que el patron coincida con el valor a comparar (match)
        //      patron => acciones a realizar,
        // en este caso guess.cmp(&secret_number) regresa ya sea
        //      Ordering::Less si guess < secret_number
        //      Ordering::Greater si guess > secret_number
        //      Ordering::Equal si guess == secret_number
    }
}
