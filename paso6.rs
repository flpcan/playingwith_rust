use std::io;

pub fn input() -> f32 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Error");
    let i : f32 = i.trim().parse().unwrap();
    return i

}
pub fn main(){

    println!("Ingresa razon a/c superior: ");
    let x1 = input();
    println!("Seleccionado : {}", x1);


    println!("Ingresa razon a/c inferior: ");
    let x2 = input();
    println!("Seleccionado : {}", x2);


    println!("Ingresa grado cemento superior: ");
    let y1 = input();
    println!("Seleccionado : {}", y1);

    println!("Ingresa grado cemento inferior: ");
    let y2 = input();
    println!("Seleccionado : {}", y2);

    println!("Ingresa grado cemento de la incognita: ");
    let y3 = input();
    println!("Seleccionado : {}", y3);



    let total = (x1*(y2-y3) + x2*(y3-y1))/(y2-y1);
    println!("El resultado razon agua cemento es : {:?}",total);

}
