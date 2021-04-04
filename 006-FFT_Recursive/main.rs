pub const PI: f64 = 3.14159265358979323846264338327950288f64;

struct Complx {
    rel: f64,
    com: f64,
}

impl Complx {
    fn new(rel: f64, com: f64) -> Complx {
        Complx { rel: rel, com: com }
    }

    fn multi(&self, y: &Complx) -> Complx {
        let nrel: f64 = (self.rel * y.rel) - (self.com * y.com);
        let ncom: f64 = (self.com * y.rel) + (self.rel * y.com);
        Complx::new(nrel, ncom)
    }

    fn sum(&self, y: &Complx) -> Complx {
        let nrel: f64 = self.rel + y.rel;
        let ncom: f64 = self.com + y.com;
        Complx::new(nrel, ncom)
    }

    fn res(&self, y: &Complx) -> Complx {
        let nrel: f64 = self.rel - y.rel;
        let ncom: f64 = self.com - y.com;
        Complx::new(nrel, ncom)
    }
}

fn recursiveFFT(a: &Vec<f64>) -> Vec<Complx> {
    let n = a.len();
    let n2 = n / 2;

    if n == 1 {
        let ss: Complx = Complx::new(a[0], 0.0);
        let mut aa: Vec<Complx> = Vec::new();
        aa.push(ss);
        aa
    } else {
        //Continuar...
    }
}

fn main() {
    let s: Complx = Complx::new(5.1, 10.2);
    let b: Complx = Complx::new(2.1, 4.6);
    let mul: Complx = s.multi(&b);
}
