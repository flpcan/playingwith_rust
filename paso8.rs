use std::io;


pub fn input() -> f32 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Error");
    let i : f32 = i.trim().parse().unwrap();
    return i
}

pub fn main() -> f32{
    println!("Ingresa valor agua: ");
    let x = input();
    println!("Ingresa valor razon: ");
    let y = input();
    println!("Dosis cemento: {}", x/y);
    return x
}
