fn main() {
    println!("Hello, world!");
    // callback funzione mia_funzione
    mia_funzione();
    altra_funzione(5);
    funzione_multiargomento(10, 'm');
    test_function();
    let cinque = cinque();
    let test = 0;
    let inc = aggiungi_uno(test);

    println!("Il valore della funzione cinque() è: {cinque}");
    println!("Il valore di inc è: {inc}");
}

// le funzioni in rust si scrivono per convenzione in snake_case (parole tutte in minuscolo con gli
// stati separati dagli underscore)
// la keyword per la dichiarazione è fn

fn mia_funzione() {
    println!("Sono Andrea.");
}

// Parametri di una funzione - per i parametri È OBBLIGATORIO DICHIARARE IL TIPO DI OGNI PARAMETRO.

fn altra_funzione(x: u32) {
    println!("Il valore di x è: {x}");
}

// facendo i test per questa ultima funzione ho scoperto che RUST pensa che si tratti di stringa se
// il testo è racchiuso tra virgolette doppie (""). Per dire al compilatore che si tratta di un
// char, occorre specificarlo tra virgolette singole.
fn funzione_multiargomento(x: u32, h: char) {
    println!("Quel mobile misura {x} {h}.");
}

// espressioni e statements
// RUST, a differenza di altri linguaggi distingue tra statement ed espressioni.
// Gli statement sono istruzioni che eseguono azioni ma non ritornano alcun valore.
// Le espressioni valutano (ritornano) un valore calcolato.
// Non è possibile assegnare uno statement in questo modo let x = (let y = 6) --> ERRORE.
// Es.
fn test_function() {
    let y = 6; // statement
               // espressione
               // NOTA BENE: IL FATTO CHE QUESTA SIA UN'ESPRESSIONE OBBLIGA A NON INSERIRE IL PUNTO E VIRGOLA
               // ALLA FINE DELL'ULTIMA VALUTAZIONE (x+1). Se lo si inserisce si trasforma l'espressione in
               // uno statement. Trasformando QUESTA ESPRESSIONE IN UNO STATEMENT, il println a fine funzione
               // restituisce errore.
    let z = {
        let x = 3;
        // NO PUNTO E VIRGOLA ALLA FINE.
        x + 1
    };
    println!("Il valore di x è: {z}");
}

// funzioni che ritornano valori
// per le funzioni che ritornano valori è NECESSARIO indicare il tipo di valore ritornato dalla
// funzione stessa. Il valore di ritorno si indica con la freccia dopo la dichiarazione della
// funzione e dei suoi parametri

// questa funzione ritorna il valore 5 che è unsigned int. Come si può notare non è indicato il
// punto e virgola perché si tratta di un espressione. È un modo di scrivere perfettamente valido
// in RUST.
fn cinque() -> u32 {
    5
}

fn aggiungi_uno(x: u32) -> u32 {
    x + 1
}
