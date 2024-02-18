// The slice type
// Gli slice permettono di referenziare una sequenza contigua di elementi in una collection
// piuttosto che l'intera collezione.
// Problema esemplificativo:
// Scrivi una funzione che accetta in ingresso una stringa formata da diverse parole separate da
// spazi e ritorna la prima parola che la funzione stessa trova nella stringa.

fn first_word(s: &String) -> usize {
    // scompongo le parole in un insieme di bytes
    let bytes = s.as_bytes();
    // imposta un ciclo for che grazie al metodo enumerate ritorna una tupla con i come indice e
    // &item come reference alla parola. Si utilizza & perché da item().enumerate() si ottiene
    // sempre una reference.
    for (i, &item) in bytes.iter().enumerate() {
        // verifica l'uguaglianza del bit con b' '
        if item == b' ' {
            // ritorna l'indice quando il ciclo arriva al primo spazio
            return i;
        }
    }
    // altrimenti ritorna la lunghezza dell'intera stringa
    s.len()
}

fn main() {
    let s = String::from("Hello World");
    let i = first_word(&s);
    println!("{}", i);
}
