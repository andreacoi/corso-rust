/* Lifetime - tempo di vita degli oggetti in Rust.
* In Rust, ogni reference ha un Lifetime. Il Lifetime è lo scope per il quale quella determinata
* reference è valida. Così come per i tipi anche le Lifetime possono essere implicite o subire
* inferenza.
* */
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}

// 10.4.5 Annotazioni dei lifetime nelle funzioni
// Le Annotazioni non hanno un grande significato di per sé. Servono solo ad indicare che la
// reference ritornata dalla funzione è valida finché ENTRAMBI i parametri della funzione stessa
// sono valide.
// L'annotazione si scrive tra virgolette angolari dopo la dichiarazione della funzione, come i
// generics utilizzando l'apostrofo e la definizione più breve possibile (solitamente a, b, c...).
