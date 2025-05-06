use std::env;
use rust_bignum::*;

fn main() -> std::io::Result<()> {
    let mut a = env::args(); a.next();
    let c = BigNum::new_with(a.next().expect("Input at least 3 parameters"));
    let d = BigNum::new_with(a.next().expect("Input at least 3 parameters"));

    match a.next().expect("Input at least 3 parameters").as_str() {
         "+" => println!("{}", (&c+&d)),
         "-" => println!("{}", (&c-&d)),
         "*" => println!("{}", (&c*&d)),
         "/" | _ => println!("No op!")
    }
    println!("\x1b[32m--------------------------\x1b[0m");
    println!("first's len: {}", c.len());
    println!("second's len: {}", d.len());
    Ok(())
}

/* use rust_bignum::*;
fn input() -> Vec<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().split_whitespace().map(|x| x.to_string()).collect()
}


fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().parse::<usize>().unwrap();

    for _ in 0..s {
        let a = input();
        println!("{}", &BigNum::new_with(a.first().unwrap().to_string()) + &BigNum::new_with(a.last().unwrap().to_string()));
    }
} */
