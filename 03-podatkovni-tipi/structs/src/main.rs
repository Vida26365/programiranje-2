struct AritmeticnoZaporedje {
    fst: i32,
    curr: i32,
    d: i32
}
struct GeometrijskoZaporedje {
    fst: i32,
    curr: i32,
    d: i32
}

use AritmeticnoZaporedje as AZ ;
use GeometrijskoZaporedje as GZ;

impl AZ {
    fn new(st: i32, k: i32) -> AZ {
        return AZ {fst: st, curr : st, d: k}
    }


    fn next(&mut self) -> i32 {
        // let dig = self.curr
        let pr = self.curr;
        self.curr += self.d;
        return pr;
    }

    fn n_th(&self, n: i32) -> i32{
        return self.fst + self.d * n
    }
    fn reset(&mut self){
        self.curr = self.fst;
    }

    fn curr(&self) -> i32 {
        return self.curr;
    }
    fn sum(&self, n: i32) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += self.n_th(i);
        }
        return s;
    }
    fn vsota(zap: &AZ, sap: &AZ) -> AZ {
        return AZ::new(zap.fst + sap.fst, zap.d + sap.d)
    }
}

impl GZ {
    fn new(st: i32, k: i32) -> GZ {
        return GZ {fst: st, curr : st, d: k}
    }


    fn next(&mut self) -> i32 {
        // let dig = self.curr
        let pr = self.curr;
        self.curr *= self.d;
        return pr;
    }

    fn n_th(&self, n: u32) -> i32{
        return self.fst.pow(n)
    }
    fn reset(&mut self){
        self.curr = self.fst;
    }

    fn curr(&self) -> i32 {
        return self.curr;
    }
    fn sum(&self, n: u32) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += self.n_th(i);
        }
        return s;
    }
    fn produkt(zap :&GZ, sap: &GZ) -> GZ {
        return GZ::new(zap.fst * sap.fst, zap.d * sap.d);
    }
}


enum BinOperacija {
    Plus,
    Minus,
    Times,
    Power
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

impl Izraz {
    fn eval(&self)-> u32 {
        match self {
            Self::Konstanta(x) => *x,
            Self::Operacija(izi, op,izd ) => match op {
                BinOperacija::Minus => (izi).eval() - (izd).eval(),
                BinOperacija::Plus => (izi).eval() + (izd).eval(),
                BinOperacija::Power => (izi).eval().pow((izd).eval()),
                BinOperacija::Times => (izi).eval() * (izd).eval(),
            }
        }
    }

    fn collect(&self) -> u32 {
        match self {
            Self::Konstanta(x) => 1,
            Self::Operacija(izi,_ ,izd ) => izi.collect() + izd.collect()
        }
    }

    fn izpis(&self) -> String {
        match self {
            Self::Konstanta(x) => x.to_string(),
            Self::Operacija(izi, op, izd) =>
                match op {
                    BinOperacija::Minus => format!("({} - {})", izi.izpis(), izd.izpis()),
                    BinOperacija::Plus =>format!("({} + {})", izi.izpis(), izd.izpis()),
                    BinOperacija::Times =>format!("({} * {})", izi.izpis(), izd.izpis()),
                    BinOperacija::Power =>format!("({} ** {})", izi.izpis(), izd.izpis())
                }
        }
    }
}

// const epskt:Izraz = Izraz(1, Plus, Izraz(2, Times, 3)); 

fn b (izraz:Izraz) -> Box<Izraz> {
    return Box::new(izraz)
}

fn main() {
    let mut zap = AZ::new(1, 1);
    print!("{}", zap.curr());
    // zap.next();
    println!("{}", zap.next());
    println!("{}", zap.curr());
    println!("{}", zap.next());
    println!("{}", zap.curr());
    zap.reset();
    println!("{}", zap.next());
    println!("{}", zap.n_th(12));
    println!("{}", zap.sum(2));
    let sap = AZ::new(7, 4);
    println!("{}", AZ::vsota(&zap, &sap).d);

    let prv = Izraz::Operacija(Box::new(Izraz::Konstanta(1)), BinOperacija::Plus, Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(2)), BinOperacija::Times, Box::new(Izraz::Konstanta(3)))));
    let drv = 
    Izraz::Operacija(Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(1)), (BinOperacija::Plus), Box::new(Izraz::Konstanta(2)))), BinOperacija::Times, Box::new(Izraz::Konstanta(3)));
    let trv = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(1)),
            BinOperacija::Plus,
            Box::new(Izraz::Konstanta(2)))),
        BinOperacija::Plus,
        Box::new(Izraz::Konstanta(3))
    );
    let str = Izraz::Operacija(
        b(Izraz::Operacija(b(Izraz::Konstanta(5)), 
        BinOperacija::Power,
        b(Izraz::Konstanta(2)))),
        BinOperacija::Plus,
        b(Izraz::Operacija(b(Izraz::Konstanta(3)), 
        BinOperacija::Power,
        b(Izraz::Konstanta(2))))
    );

    let pet = Izraz::Operacija(
        b(Izraz::Operacija(
            b(Izraz::Konstanta(5)), 
            BinOperacija::Times, 
            b(Izraz::Konstanta(5)))), 
        BinOperacija::Plus,
        b(Izraz::Operacija(
            b(Izraz::Konstanta(4)), 
            BinOperacija::Power, 
            b(Izraz::Konstanta(2)))));

    println!("{}", prv.eval());
    println!("{}", drv.eval());
    println!("{}", trv.eval());
    println!("{}", str.eval());
    println!("{}", pet.eval());
    println!("{}", pet.collect());
    println!("{}", prv.collect());
    println!("{}", pet.izpis());
}
