// # Pacchetti e crates
// I crate possono essere di due tipi:
//  - crate binari
//  - crate libreria
//  I crate binari sono i programmi che compiliamo con cargo o con rustc (indifferentemente).
//  I crate binari possono essere compilati E DEVONO SEMPRE avere un main. Le librerie invece,
//  non possono essere compilate.
//  Tutti i Rustaceans si riferiscono, con il termine crate, alle librerie. Ma si possono anche
//  riferire ai binari in maniera del tutto intercambiabile.
//  Il crate root è il file da cui parte il nostro programma rust. In questo specifico caso, quello
//  che stai leggendo è il nostro crate root.
//  La struttura dei moduli rust è fortemente gerarchica. Infatti occorre seguire una precisa
//  struttura di cartelle che si traduce in:

use crate::garden::vegetables::Melanzana;

pub mod garden;
fn main() {
    let pianta_buonissima = Melanzana {};
    println!(
        "La pianta più buona del mondo sta crescendo: {:?}",
        pianta_buonissima
    );
}
