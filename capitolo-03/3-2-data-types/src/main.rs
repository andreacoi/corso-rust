use core::f64;
use std::io;

fn main() {
    // la funzione parse è in grado di eseguire il cast solo specificato nell'annotazione tipo
    // inserita dopo il nome della variabile, vedi u32.
    let guess: u32 = "42".parse().expect("Not a number");

    // tupla - presta attenzione al modo in cui vengono annotati i diversi tipi presenti nella
    // tupla. la sintassi è utilizzando i due punti e successivamente i tipi rinchiusi tra
    // parentesi.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destrutturazione
    // questo tipo di assegnazione si chiama destrutturazione: significa che la tupla iniziale
    // viene spezzettata in tre parti accessibili singolarmente.
    let (x, y, z) = tup;
    println!("Il valore di y è: {y}");

    // array - gli array in rust hanno sempre lunghezza fissa. diversamente dal php è possibile
    // utilizzare i vettori per permettere un'allocazione dinamica.
    // modi di dichiarazione
    let a = [1, 2, 3, 4, 5];
    // dichiarazione concisa
    let b = [3; 5];
    //è uguale a
    let b = [3, 3, 3, 3, 3];
    //è possibile anche annotare il tipo delle variabili contenute nell'array
    // quando si annota il tipo delle variabili va annotato OBBLIGATORIAMENTE anche il numero delle
    // variabili contenute nell'array.
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // LOGICHE DI ACCESSO ALLO STACK
    // le logiche di accesso allo stack sono identiche a quelle solite utilizzate con altri
    // linguaggi
    let first = b[0]; // 1
    let second = b[1]; // 2

    let test = [1, 2, 3, 4, 5];

    // snippet di codice che serve per verificare le modalità di accesso a un indice di un array.
    // In molti linguaggi di basso livello quando si prova ad accedere a un indice fuori dallo
    // stack, il linguaggio stesso prova ad accedere a quella "porzione" di memoria. In rust,
    // invece, si subisce un blocco a prescindere e il codice invoca il panic.
    println!("Inserisci un indice a cui accedere:");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Non valido");

    let index: usize = index
        .trim()
        .parse()
        .expect("Non è stato inserito un numero, che fai?");

    let element = test[index];
    println!("Elemento selezionato: {element}");
}
