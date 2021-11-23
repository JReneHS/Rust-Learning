//Struct tipo C
struct Person {
    name: String,
    age: i32,
    id: String,
    like_mangos: bool,
    punto: Point3D,
}

//Struct tipo Tupla
struct Point3D(u32, u32, u32);

//Struct tipo unidad
//struct Unit;

//Enums
enum Desigualdad {
    Mayor(i32),
    Menor(i32),
    Igual(i32),
    Diferente(String, String),
    Centros(Point3D, Point3D),
}

impl Person {
    fn comparar_edad(&self, per: &Person) -> Desigualdad {
        if self.age > per.age {
            Desigualdad::Mayor(self.age - per.age)
        } else if self.age < per.age {
            Desigualdad::Menor(per.age - self.age)
        } else {
            Desigualdad::Igual(self.age)
        }
    }

    fn comparar_nombre(&self, per: &Person) -> Desigualdad {
        if self.name == per.name {
            Desigualdad::Igual(666_666)
        } else {
            Desigualdad::Diferente(self.name.clone(), per.name.clone())
        }
    }

    fn comparar_centros(&self, per: &Person) -> Desigualdad {
        if self.punto.0 == per.punto.0 && self.punto.1 == per.punto.1 && self.punto.2 == per.punto.2
        {
            Desigualdad::Igual(777_777)
        } else {
            let a: Point3D = Point3D(self.punto.0, self.punto.1, self.punto.2);
            let b: Point3D = Point3D(per.punto.0, per.punto.1, per.punto.2);
            Desigualdad::Centros(a, b)
        }
    }

    fn prin(&self, des: Desigualdad) {
        match des {
            Desigualdad::Mayor(y) => println!("Mayor por {} anios de diferencia", y),
            Desigualdad::Menor(y) => println!("Menor por {} anios de diferencia", y),
            Desigualdad::Igual(y) => {
                if y == 666_666 {
                    println!("Tienen el Mismo Nombre {}", self.name);
                } else if y == 777_777 {
                    print!("Tienen el mismo centro ");
                    println!(
                        "(x:{}, y:{}, z:{})",
                        self.punto.0, self.punto.1, self.punto.2
                    );
                } else {
                    println!("Iguales con {} de edad", y)
                }
            }
            Desigualdad::Diferente(x, y) => println!("Tienen diferente nombre: {}, {}", x, y),
            Desigualdad::Centros(x, y) => {
                print!("Tienen Diferentes centros: ");
                print!("(x:{}, y:{}, z:{}) ", x.0, x.1, x.2);
                println!("(x:{}, y:{}, z:{})", y.0, y.1, y.2);
            }
        }
    }
}

fn main() {
    let persona01 = Person {
        name: String::from("Rene"),
        age: 25,
        id: "2016630187".to_string(),
        like_mangos: true,
        punto: Point3D(0, 0, 0),
    };

    let persona02 = Person {
        name: String::from("Joke"),
        age: 30,
        id: persona01.id.clone(),
        punto: Point3D(10, 12, 11),
        ..persona01 //Rellena los demas campos con los datos de persona01
    };

    let origen = Point3D(0, 0, 0);

    //let unit = Unit;

    if persona01.like_mangos {
        println!(
            "{:?} tiene {:?} anios y le gustan los mangos y su ID es {:?}.",
            persona01.name, persona01.age, persona01.id
        );
    } else {
        println!(
            "{:?} tiene {:?} anios y NO le gustan los mangos y su ID es {:?}.",
            persona01.name, persona01.age, persona01.id
        );
    }
    println!(
        "{} tiene {} su Id es {} y {}",
        persona02.name, persona02.age, persona02.id, persona02.like_mangos
    );
    println!("(x:{}, y:{}, z:{})", origen.0, origen.1, origen.2);

    persona01.prin(persona01.comparar_nombre(&persona02));
    persona01.prin(persona01.comparar_centros(&persona02));
    persona01.prin(persona01.comparar_edad(&persona02));
}
