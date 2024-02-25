// standard library - utilizzata per le funzioni standard di i/o
// questa dichiarazione si chiama PRELUDE
use rand::Rng;
use std::cmp::Ordering;
use std::{io, num};

fn main() {
    println!("Indovina il numero");
    let numero_segreto = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Inserisci un numero per iniziare il gioco!");
        //  In Rust, variables are immutable by default - !!IMPORTANTE!! per rendere la variabile mutabile
        //  occorre inserire mut dopo la keyword let
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Lettura riga fallita.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Rust has a strong, static type system. È un linguaggio fortemente tipizzato.
        // eseguo il cast tra String (tipo dichiarato in lettura) e u32 (integer)
        // i due punti tra la ridichiarazione della variabile (shadowing) servono per una annotazione
        // necessaria per il tipo atteso in fase di casting. Sostanzialmente indica a parse la
        // "destinazione" del cast (u32, infatti indica un intero senza segno)
        println!("Hai inserito {guess}");
        match guess.cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo poco!"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            }
            Ordering::Greater => println!("Troppo!"),
        }
    }
}
