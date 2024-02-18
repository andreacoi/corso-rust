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
// questa funzione può essere riscritta utilizzando gli slice in questo modo:
//
fn first_word_with_slices(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    // rendo mutable la stringa
    let s = String::from("He Man");
    let word = first_word(&s);

    let real_word = first_word_with_slices(&s);    // word rimane uguale a 5! Questo non va bene.
    println!("{}", word);
    println!("{}", real_word);

    let sliced = slicer(&s);
    println!("{}", sliced);
    // resta beninteso che gli slice possono essere composti anche da numeri, come in questo caso:
    let a = [1,2,3,4,5];
    // ovviamente è possibile accedervi in questo modo, specificando index_iniziale e index_finale
    // saranno utili più in avanti parlando dei vettori.
    let slice = &a[1..3];
    // come da documentazione - non l'ho capita
    assert_eq!(slice, &[2, 3]);
    // le stringhe letterali sono anche un tipo di slice ed è per questo che sono immutabili. In
    // alcuni casi sono anche più facili da trattare per via della dichiarazione immediata che li
    // caratterizza.
}
// imposto funzione che ritorna una parte di una stringa attraverso il tipo slice. Il tipo slice si
// dichiara, come si può vedere in basso come &str.
// Il tipo slice funziona esattamente come gli array di caratteri per il PHP, uniti costituiscono
// una stringa, ma possono essere separati tramite l'indicazione dell'indice.
fn slicer(mia_stringa: &String) -> &str {
    let sliced = &mia_stringa[0..3]; // è uguale a let sliced = &mia_stringa[..3]; lo stesso vale
    // per il trailing number finale let sliced = &mia_stringa[0..]
    sliced
}
