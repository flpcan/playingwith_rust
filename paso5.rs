//Valor resistencia media requerida
// fr = fc + t x s
//compactcacion

use std::io;
pub fn main(){

    let fc = grado();
    let t = fraccion();
    let s = condiciones();
    let fr = fc + (t * s);
    println!("Valor total fr es: {}", fr);

}

fn grado() -> f32 {
        println!("Ingresa valor Grado :");
    let grado = input();

    return grado
}
fn fraccion() -> f32 {
    println!("Ingresa valor t:");
    let fraccion = input();

    return fraccion
}
fn condiciones() -> f32 {
    println!("Ingresa valor condiciones:");
    let condiciones = input();

    return condiciones
}



pub fn input() -> f32 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Error");
    let i : f32 = i.trim().parse().unwrap();
    return i

    }
