// con questo programma (e i suoi vari refactoring) sarà possibile capire quando utilizzare le
// struct.
// Iniziamo con il creare un programma banale per calcolare l'area di un rettangolo (b x h).

/* VERSIONE 0.1 fn main() {
    let width = 50;
    let height = 30;
    println!("L'area del rettangolo è: {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}  === FINE VERSIONE 0.1 */

// in questo piccolo snippet di codice non si evince in nessun punto che le misure con le quali
// calcoliamo l'area siano "collegate" o afferenti ad un unico rettangolo. Pertanto possiamo
// riscrivere il programma utilizzando le tuple, con l'obiettivo di renderlo maggiormente chiaro.
//
// Refactoring utilizzando le TUPLE.
// In questo modo il nostro programma è diventato più strutturato utilizzando le tuple, di contro
// perde di chiarezza non potendo nominare in maniera precisa i campi della tupla e dovendo
// affidarci ai suoi indici per recuperare i dati.
/* VERSIONE 0.2
fn main() {
    let rettangolo1 = (50, 30);
    println!("L'area del rettangolo è {}", area(rettangolo1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} */

// REFACTORING CON LE STRUCT - VERSIONE 0.3

fn main() {
    let rettangolo = Rettangolo {
        width: 50,
        height: 30,
    };
    // ammettiamo di voler visualizzare i dati del rettangolo in questione prima di eseguire il
    // calcolo dell'area. Questo può essere utile per dare delle informazioni all'utente che
    // utilizza il programma.
    // Ma come stampare una STRUCT?
    // Tutti i tipi primitivi implementano il metodo Display (std::fmt::Display) perché tutti i
    // tipi primitivi hanno un solo modo di essere visualizzati. Le struct invece possono essere
    // stampate in diversi modi: con le parentesi, senza, con le virgole, con tutti i campi o solo
    // con alcuni. Pertanto, utilizzare la macro println! produce un errore in fase di
    // compilazione.
    // Rust, infatti, non prova ad indovinare COME comportarsi quando si tratta di stampare la
    // struct.
    // Come possiamo fare? Fortunatamente possiamo usare questa formattazione:
    //
    // println!("Dettagli rettangolo: {:?}", rettangolo);
    //
    // Questo tipo di sintassi utilizza un tipo di sintassi in output chiamato Debug. Il trait
    // Debug è utile per stampare la nostra struct in un modo chiaro per gli sviluppatori, mentre
    // si esegue il debug del nostro codice.
    // Per abilitare lo specificatore :? è necessario dichiarare l'attributo esterno #[derive(Debug)] APPENA PRIMA DELLA DEFINIZIONE DELLA STRUCT.
    // N.B. Lo specificatore può essere dichiarato anche con l'hashbang per permettere una migliore leggibilità e formattazione.
    // Es. {:#?}
    //
    println!("Dettagli rettangolo: {:?}", rettangolo);
    println!("L'area del rettangolo è: {}", area(&rettangolo));
}

fn area(rettangolo: &Rettangolo) -> u32 {
    rettangolo.width * rettangolo.height
}

#[derive(Debug)]
struct Rettangolo {
    width: u32,
    height: u32,
}
