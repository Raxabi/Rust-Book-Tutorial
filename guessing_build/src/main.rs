use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Introduce un numero: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Caracter inesperado: ");

        let random_number = rand::thread_rng().gen_range(1..=100);

        println!("El numero aleatorio es {random_number}");

        let guess: u32 = guess.trim().parse().expect("Por favor introduce un numero");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
        
        println!("Desea gunar de nuevo?");
        println!("S / N: ");
        let mut continue_game = String::new();

        io::stdin().read_line(&mut continue_game).expect("Se esperaba 'S' o 'N'");
    }
}