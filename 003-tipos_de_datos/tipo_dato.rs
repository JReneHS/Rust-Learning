fn main() {
    println!("Tipos de datos en Rust");
    println!("Tipos Primitivos Simples");
    println!(
        "  >> Integers\n  >>  Unsigned Integers\n  >>  Floating-Point\n  >>  Bool\n  >>  Char"
    );
    // Signed integers
    // max value >> 2^(n-1) - 1
    // min value >> -2^(n-1)
    let byte: i8 = 127;
    let dbyte: i16 = 32_767;
    let int: i32 = 2_147_483_647;
    let long: i64 = 9_223_372_036_854_775_807;
    let long_long: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

    // Unsigned integers
    // max value >> 2^n - 1
    // min value >> 0 (lol)
    let unsigned_byte: u8 = 255;
    let unsigned_dbyte: u16 = 65_535;
    let unsigned_int: u32 = 4_294_967_295;
    let unsigned_long: u64 = 18_446_744_073_709_551_615;
    let unsigned_long_long: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    // Se puede hacer declaraciones con sufijos
    // let nn = 128u16

    println!("\nMAX_I8 = {}", byte);
    println!("MAX_I16 = {}", dbyte);
    println!("MAX_I32 = {}", int);
    println!("MAX_I64 = {}", long);
    println!("MAX_I128 = {}", long_long);

    println!("\nMAX_U8 = {}", unsigned_byte);
    println!("MAX_U16 = {}", unsigned_dbyte);
    println!("MAX_U32 = {}", unsigned_int);
    println!("MAX_U64 = {}", unsigned_long);
    println!("MAX_U128 = {}", unsigned_long_long);

    println!("\nLa funcion wrapping_add para evitar el overflow reseteando el bit");
    println!("MAX_U8 + 1 = {}", unsigned_byte.wrapping_add(1));
    println!("MAX_U8 + 2 = {}", unsigned_byte.wrapping_add(2));
    println!("MAX_U8 + 3 = {}", unsigned_byte.wrapping_add(3));
    println!("MAX_U8 + 4 = {}", unsigned_byte.wrapping_add(4));
    println!("\nLa funcion checked_add envia un option");
    println!("MAX_U8 + 0 = {:?}", unsigned_byte.checked_add(0));
    println!("MAX_U8 + 1 = {:?}", unsigned_byte.checked_add(1));
    println!("\nLa funcion saturating_add da el numero mas cercano");
    println!("MAX_U8 + 1 = {:?}", unsigned_byte.saturating_add(1));

    // Rust no hay conversion de tipos implicitos D:
    // se usa la palabra clave as
    // var1 + var2 as u32

    //Valores de tipo Flotante
    let float: f32 = 65.5;
    let double: f64 = 4596.6665;
    println!("\nPrimitivos tipo coma Flotante");
    println!("Float >> {}", float);
    println!("Double >> {}", double);

    //Formas de Representar numeros
    //Decimal >> 23_566
    //Hex >> 0xff
    //Octal >> 0o77
    //Binario >> 0b1111_0000
    //Byte(solo en u8) >> b'A' (segun el codigo ASCII A = 65)

    //Booleans
    let verdadero: bool = true;
    let falso: bool = false;
    //Como no hay cast de tipos aqui true es diferente de 1
    //y false es diferente a 0
    //SOLO SE PUEDE PASAR DE BOOL A INTEGER.
    assert_eq!(verdadero as i32, 1);
    assert_eq!(falso as i32, 0);

    //variables tipo char
    let caracter: char = 'A';
    println!("\nPrimitivos tipo Char Unicode");
    println!("Caracter >> {}", caracter);
    //Los char son variables de tipo Unicode
    //OJO que las cadenas NO son Colecciones de Char
    //Son coleccienes de bytes tipo UTF-8
    //Por ende char != Strings

    //Variables de tipo Compuesto
    println!("\nTipos Primitivos Compuestos");
    println!("  >>  Tuplas\n  >>  Arrays");
    //Las Tuplas son conjuntos de valores mixtos
    //SON estaticos, es decir una vez definido no se puede aumentar ni disminuir
    //NO se puede ingresar mediante indice
    let tupla: (i32, &str, bool) = (158, "holi", true);
    println!("\nPara ingresar a una tupla se usa numeros como elemento:");
    println!("Tupla: (i32, &str, bool) = (158, 'holi', true)");
    println!("tupla.0 >> {}", tupla.0);
    println!("tupla.1 >> {}", tupla.1);
    println!("tupla.2 >> {}", tupla.2);

    //Se puede aprovechar es Patern Matching para definir nombres a las tuplas pero no se
    //empaquetan en un conjunto.
    println!("\nPatern Matching con tuplas:");
    println!("ss = 'Hola Mundo BB 7u7r';");
    println!("let (s1, s2) = ss.split_at(11);");
    let ss: &str = "Hola Mundo BB 7u7r";
    let (s1, s2) = ss.split_at(11); //Tupla sin indicar valores
    println!("s1 >> '{}'\ns2 >> '{}'", s1, s2);

    println!("\nTupla separada con variables");
    println!("let tup = (12, 'Haa', false)");
    let tup = (12, "Haa", false);
    println!("let (xx, yy, zz) = tup;");
    let (xx, yy, zz) = tup;
    println!("xx >> {}", xx);
    println!("yy >> {}", yy);
    println!("zz >> {}", zz);
    //Tuplas tipo Unit o Cero
    //let cero: () = ();
    //Es lo que se devuelve por defecto cuando NO "retornan" nada
    //ejemplo
    //
    //      fn f(n: i32) {
    //          println!("{}", n);
    //      }
    //
    //      fn f(n: i32) -> () {
    //          println!("{}", n);
    //      }
    //
    //      fn f(n: i32) {
    //          println!("{}", n);
    //          () si no hay ';' se toma como return
    //      }
    //      fn f(n: i32) {
    //          println!("{}", n);
    //          return ();
    //      }

    //Arrays
    println!("\nTipos Compuestos Arrays");
    //Los arrays son igual fijos o Estaticos
    //Lo unico dinamico son los vectores
    let arr1 = [u8; 1024];
    //Declaracion explisita con longitud especifica
    let arr2: [i16; 5] = [1, 2, 3, 4, 5];
    //Declaracion implicita
    let arr3 = [10, 9, 1];
    println!("{:?}", arr2);
    println!("{:?}", arr3);

    //Para acceder a los elementos de un array siempre se usa una variablo tipo usize

    let pos: usize = 2;
    println!("el elemento en la pos {} es >> {}", pos + 1, arr2[pos]);
    println!("{:?}", arr1.get(pos));
}
