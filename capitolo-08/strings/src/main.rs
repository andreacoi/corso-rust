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
}
