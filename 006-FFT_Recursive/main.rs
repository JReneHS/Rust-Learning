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

fn rFFT(x: &Vec<Complx>, inv: i32) {
    let n: usize = x.len();
    let mut i: usize = 1;
    let mut j: usize = 0;
    let mut k: usize = n >> 1;
    while i < n - 1 {
        j ^= k;
        while j < k {
            if i < j {
                //continuar con el swap
            }
            k >>= 1;
            j ^= k;
        }
        i += 1;
    }
}

fn main() {
    let inv: i32 = 10;
    let mut arr: Vec<Complx>;
    rFFT(&arr, inv);
}
