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
    println!("L'area del rettangolo è: {}", area(&rettangolo));
}

fn area(rettangolo: &Rettangolo) -> u32 {
    rettangolo.width * rettangolo.height
}

struct Rettangolo {
    width: u32,
    height: u32,
}
