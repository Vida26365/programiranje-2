use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::format;
use std::ops::Div;
use std::ops::Sub;
use std::ops::{Add, Mul};
use std::fmt::Display;
use std::process::Output;
use std::thread::panicking;



trait Sequence<T> {
    fn name(&self) -> String;
    fn n_th(&self, k:usize) ->Option<T>;
    fn contains(&self, item: T) -> bool;
    fn start(&self) -> T;
}

struct C<T> {
    c: T
}
struct Ci64 {
    c: i64
}

impl <T> C<T> {
    fn new(k:T) -> C<T> {
        C{c:k}
    }
}

impl <T: Copy + Eq> Sequence<T> for C<T> {
    fn name(&self) -> String {
        format!("Constanta")
    }
    fn n_th(&self, k:usize) ->Option<T> {
        return Some(self.c);
    }
    fn start(&self) -> T {
        return self.c;
    }
    fn contains(&self, item: T) -> bool {
        return item == self.c;
    }
}


struct AZ<T> {
    fst: T,
    curr: T,
    d: T
}

impl <T: Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>> Sequence<T> for AZ<T> {
    fn name(&self) -> String {
        format!("Aritmetično zaporedje")
    }
    fn n_th(&self, n:usize) ->Option<T> {
        let mut s = self.fst;
        for i in 1..n {
            s = s + self.d
        }
        return s
    }
    fn start(&self) -> T {
        self.fst
    }
    fn contains(&self, item: T) -> bool {
        // (item - self.fst) % self.d
        panic!()
    }
}

struct GZ {
    fst: i32,
    curr: i32,
    d: i32
}

impl Sequence<i32> for GZ {
    fn name(&self) -> String {
        format!("Geometrično zaporednje")
    }
    fn n_th(&self, n:usize) ->Option<i32> {
        let mut s = self.fst;
        for i in 1..n {
            s = s*self.d
        };
        return Some(s);
    }
    fn start(&self) -> i32 {
        self.fst
    }
    fn contains(&self, item: i32) -> bool {
        panic!()
    }
}

// use AritmeticnoZaporedje as AZ ;

// impl <T: Copy + Add<Output = T> + Mul<Output = T> + Eq> AZ<T> {
//     // fn name(&mut self) -> String {
//     //     return format!({"Aritmetčno zaporednje tipa {} ({}{})", T, self.fst, self.d})
//     // }

//     fn new(st: T, k: T) -> AZ<T> {
//         return AZ {fst: st, curr : st, d: k}
//     }


//     fn next(&mut self) -> T {
//         let pr = self.curr;
//         self.curr = self.curr + self.d;
//         return pr;
//     }

//     fn n_th(&self, n: u32) -> T{
//         let mut s = self.fst;
//         for i in 1..n {
//             s = s + self.d
//         }
//         return s
//         // panic!()
//         // return self.fst + self.d * n
//     }
//     fn reset(&mut self){
//         self.curr = self.fst;
//     }

//     fn curr(&self) -> T {
//         return self.curr;
//     }
//     fn sum(&self, n: u32) -> T {
//         // let mut s = 0;
//         let mut s = self.fst;
//         for i in 1..n {
//             s = s + self.n_th(i);
//         }
//         return s;
//     }
//     fn vsota(zap: &AZ<T>, sap: &AZ<T>) -> AZ<T> {
//         return AZ::new(zap.fst + sap.fst, zap.d + sap.d)
//     }
// }

impl<T: Eq> PartialEq for AZ<T>{
    fn eq(&self, other: &Self) -> bool {
        return self.d == other.d && self.fst == other.fst
    }
}

fn main() {
    let mut zap = AZ::new(1, 1);
    let mut zbp = AZ::new(1, 1);
    // println!(format!("{}", zap == zbp));
}



