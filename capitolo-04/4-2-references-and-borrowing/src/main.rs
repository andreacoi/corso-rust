fn main() {
    let mut stringa =
        String::from("Nel mezzo del cammin di nostra vita, mi ritrovai per una selva oscura");
    // il simbolo & (ampersand) rappresenta il referencing, cioè un puntamento a un indirizzo di
    // memoria che contiene il riferimento al valore della variabile.
    // Possiamo utilizzare & senza "prendere possesso della variabile passandola allo scope della
    // funzione di destinazione."
    let lunghezza = calcola_lunghezza(&stringa);
    println!("La lunghezza del versetto: {} è {}", stringa, lunghezza);

    change(&mut stringa);
    println!("{}", &stringa);

    // utilizzando le reference mutable abbiamo però una grande limitazione: se si associa una
    // reference mutable a un valore non è possibile associarne degli altri. Non è possibile prendere in
    // prestito una referenza mutable ad una variabile più di una per volta.
    // Es. 

    let mut another_string = String::from("ciao come stai?");
    let r1 = &mut another_string;
    //    let r2 = &mut another_string; // ERRORE!

    println!("r1: {}", r1);

    // Questo tipo di vincolo esiste per scongiurare collisioni e race condition nel caso di
    // variazioni contemporanee degli stessi dati nello stesso dato intervallo di tempo t.
    // Visto che le data races possono causare comportamenti imprevisti, RUST, semplicemente, si
    // rifiuta di eseguire la compilazione laddove si presenti una situazione di data race.
    //
    // Resta comunque possibile accedere a riferimenti mut multipli in porzioni di codice delimitate dalle parentesi graffe.
    // Es.
    {
        let r2 = &mut another_string;
        println!("r2: {}", r2);
    }
    // come si può vedere in questo snippet RUST non restituisce errore perché le due dichiarazioni non
    // sono simultanee (lo scope è diverso).
    // Il linguaggio impedisce anche questioni "miste", restituisce, infatti, errore se si
    // combinano reference mutable e immutable. Questo perché si cerca di garantire la maggior
    // stabilità possibile. Quando infatti si fa riferimento ad una reference immutable ci si
    // aspetta che questa non cambi. Se si miscelano i riferimenti, tipo uno mutable e uno
    // immutable potrebbe succedere che in un punto del codice non specificato questo riferimento
    // cambi valore e questo viene impedito.
    // 
    // rif. Users of an immutable reference don’t expect the value to suddenly change out from under them! 
    // However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    //
    // SCOPE DI UNA REFERENCE 
    // Lo scope di una reference inizia con la sua dichiarazione e termina con il suo ultimo utilizzo.
    // Di seguito infatti dichiaro r3 = &mut another_string. Lo posso fare perché lo scope della reference ad another_string è terminato alla riga 23, invocando println!
    // IMPORTANTE! GLI SCOPE, QUINDI, NON SI SOVRAPPONGONO, PER CUI IL CODICE È VALIDO.
    let r3 = &mut another_string;
    println!("{}", r3);

    let r4 = dangle();

    println!("{}", r4);
}

    // il fatto che nell'argomento della funzione venga utilizzato & ci fa capire che stringa è una
    // reference essa stessa. Il fatto che usi una reference ci garantisce che il suo valore non venga
    // distrutto dopo essere stato utilizzato. Per lo stesso motivo non abbiamo l'obbligo di restituire
    // la proprietà alla funzione chiamante. Tradotto in termini ancora più semplici:
    // I REFERENCE NON HANNO OWNERSHIP.
    // Il referencing, in RUST, prende il nome di BORROWING (prendere in prestito).
fn calcola_lunghezza(stringa: &String) -> usize {
    let lunghezza = stringa.len();
    lunghezza
}

// Cosa succede però, se tentiamo di modificare qualcosa che abbiamo preso in prestito?

// fn change(old_same_string: &String) {
// old_same_string.push_str(", che la diritta via era smarrita.")
// }
// Questa funzione va in errore perché così come le variabili sono IMMUTABILI DI DEFAULT, lo stesso
// vale per le reference. È quindi possibile scrivere questa funzione dichiarando mut affianco
// all'ampersand.
// In questo modo:

fn change(old_same_string: &mut String) {
    old_same_string.push_str(", che la diritta via era smarrita");
}

// PUNTATORI PENDENTI - DANGLING REFERENCES

/* fn dangle() -> &String {
    let s = String::from("hello!");
    &s
} */
// in questa funzione si ritorna una reference a una stringa che però viene distrutta alla chiusura
// della funzione. Di fatto quindi il puntatore "punta" a una porzione di memoria distrutta o
// inaccessibile perché lo scope è terminato.
// Allo stesso modo, come tutto il resto, RUST impedisce la compilazione in casi di questo tipo
// salvaguardando eventuali accessi a porzioni di memoria disallocate precedentemente.
// In questa situazione, quindi, è meglio ritornare direttamente la stringa in questo modo:

fn dangle() -> String {
    let s = String::from("hello!");
    s
}

