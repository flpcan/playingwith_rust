use std::io;

pub fn main() {

    let x = 1000.0 - agua() - aire() - dosiscemento();
    println!("Dosis arido : {}", x);

}

fn aire() -> f32 {
    println!("Ingresa valor aire");
    let a = input();
    println!("Aire es {}", a);
    return a
}
fn agua() -> f32 {
    println!("Ingresa valor agua");
    let y = input();
    println!("Agua es {}", y);
    return y
}

fn dosiscemento() -> f32{
    println!("Dosis cemento:");
    let x = input();
    println!("Dosis cemento ingresada: {}",x);
    return x
}

pub fn input() -> f32 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Error");
    let i : f32 = i.trim().parse().unwrap();
    return i

}
