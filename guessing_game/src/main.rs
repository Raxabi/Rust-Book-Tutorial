use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("El numero secreto es: {secret_number}");

    print!("Por favor introduce tu guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer la linea");

    println!("Has introducido: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Demasiado pequeño")
    }
}