fn main() {
    // Per gestire gli errori irrecuperabili, spesso sintomo di bug, Rust usa panic!
    // Proviamolo.
    // Rust non gestisce le eccezioni, ma ha ben due modalità di gestire gli errori:
    //  - quelli irrecuperabili vengono gestiti con la macro panic!
    //  - quelli recuperabili vengono gestiti con Result<T, E>
    //    panic!("Brucia intra lu Salentu.");
    //  Un altro modo per causare il panic è provare ad accedere (ad esempio) ad un indice di un
    //  vettore che non esiste.
    //  Proviamo.
    let vettore_di_prova = vec![1,2,3];
    // questo è un vettore di tre elementi.
    // Proverò ad accedere al centesimo indice. (I VETTORI HANNO L'INDICE CHE INIZIA DA 0).
    vettore_di_prova[100];
}
