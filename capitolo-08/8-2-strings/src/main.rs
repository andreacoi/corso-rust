fn main() {
    // # Immagazzinare testo codificato in UTF-8 con le String.
    // Cos'è una stringa?
    // I rustaceans si riferiscono alle stringhe in due casi:
    // - quando si parla di String vera e propria
    // - quando si parla di string slices (&str)
    // Una stringa funziona essenzialmente come un vettore, infatti per dichiarare una stringa
    // vuota è sufficiente utilizzare tale sintassi (che è molto simile a quella utilizzata) per
    // dichiarare un vettore vuoto:
    // Stringa vuota
    let mut stringa = String::new();

    // Encoding UTF-8
    // In rust, tutte le stringhe sono codificate in UTF-8.
    //
    // Le stringhe possono essere concatenate con il + e con la macro format!
    // La questione ownership, per le stringhe è molto particolare:
    // push_str, ad esempio è una funzione che consente di appendere dei parametri alla stringa,
    // senza "prenderne possesso", perché utilizzato passando la stringa sorgente e quella da
    // aggiungere come parametri della funzione stessa.
    //
    // # Accedere a caratteri singoli di una stringa utilizzando gli indici
    // In RUST, È IMPOSSIBILE ACCEDERE A UNA STRINGA O A PARTE DI ESSA UTILIZZANDO GLI indici
    let s2 = String::from("Ciao");
    // s2[1] infatti restituirà un errore. Questo essenzialmente succede per due motivi:
    // 1. Le modalità di storage delle stringhe
    // 2. Il fatto che essenzialmente l'elemento "stringa", in Rust rappresenti un Wrapper attorno
    //    a un Vec<u8>.
    // REGOLA GENERALE
    // MAI UTILIZZARE GLI INDICI PER SPEZZETTARE UNA STRINGA ED ACCEDERE AI RELATIVI VALORI.
    // RUST stesso fornisce dei suggerimenti (DEGLI AVVERTIMENTI!) su come NON COMPORTARSI nel caso
    // in cui si voglia effettuare lo slice di una stringa con gli indici.
    // Esistono tuttavia dei metodi per eseguire un iterazione su una Stringa.
    // Uno può essere:
    //
    for c in s2.chars() {
        println!("{}", c);
    }
    // questo per visualizzare carattere per carattere la stringa s2.
    // Se invece volessimo visualizzare i bytes, esiste un metodo anche per quello:
    for b in s2.bytes() {
        println!("{}", b);
    }
}
