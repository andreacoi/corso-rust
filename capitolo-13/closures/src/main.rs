use std::{thread, time::Duration};

// 13.1 Le closure
// Le closure sono delle funzioni anonime che possono "salvate" in una variabile o passate come
// argomenti di una funzione. La closure può essere creata in un punto del codice e poi può esser
// valutata dovunque, in differenti contesti (context).
// Diveramente dalle funzioni però, le closure possono "catturare" valori a partire dallo scope nel
// quale sono definite.
// Per approfondire il concetto sviluppiamo un applicativo che si occupa di gestire un giveaway di
// magliette di due colori diversi fatto da un'azienda che commercia magliette.
//
// DESCRIZIONE APPLICATIVO
// L'applicazione dovrebbe funzionare in questo modo: le persone (clienti) che sono iscritte alla
// mailing list aziendale possono scegliere una maglietta di un colore. Nel caso in cui vengano
// sorteggiati come vincenti per il giveaway l'azienda invierà loro una maglietta dello stesso
// colore di quello settato nell'area personale. Qualora l'utente NON dovesse settare alcun colore
// e questo utente dovesse venir sorteggiato come vincente per il giveaway, l'azienda invierà al
// vincitore SENZA COLORE PREFERITO la maglietta con la maggior giacenza in magazzino.
//
// COME REALIZZARLA
// Possiamo partire con una enum che elenchi i colori di magliette disponibili, che limiteremo a
// blu e rooso.
// Poi abbiamo bisogno di una struct che simuli il magazzino avendo come unico campo "shirts" che è
// un vettore composto di magliette di colore diverso.
// Un altro componente importante è una funzione che ci faccia sapere la maglietta con la maggiore
// giacenza in magazzino.
// All'interno del main, faremo i controlli per l'estrazione della maglietta.
//
// Funzioni associate
// La funzione giveaway e quella most_stocked sono funzioni associate di Inventory.
#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
// Dichiaro le funzioni associate a Inventory.
impl Inventory {
    // argomenti di giveaway:
    // &self -> Inventory
    // user_preference: UNA delle Opzioni di ShirtColor -> Option<ShirtColor>
    //
    // Ritorna:
    // ShirtColor
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // ritorno user_preference con unwrap_or_else
        // Le pipe indicano una closure, tra le pipe vanno inseriti gli argomenti della closure,
        // qualora ne abbia.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // argomenti di most_stocked:
    // &self --> Inventory
    // Ritorna:
    // enum di tipo ShirtColor
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}
fn main() {
    // num è l'argomento della closure, infatti viene indicato tra le pipe.
    // questo è un test di scrittura per esteso della closure, infatti ho specificato anche il tipo
    // di ritorno. N.B. Questa sintassi è corretta ed è formalmente valida. Tuttavia non è
    // necessaria perché le closure sono in grado di praticare inferenza per quasi tutti i tipi di
    // variabile.
    let expensive_closure = |num: u32| -> u32 {
        println!("Calcolo eseguito molto lentamente...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // classico esempio di una closure base.
    // 1. si specifica una variabile con il let come una normalissima dichiarazione di variabile
    // 2. SUBITO DOPO L'UGUALE si inserisce l'espressione da valutare. Senza parentesi se solo una
    //    riga, con le parentesi se più di una riga;
    // 3. Per il callback è possibile utilizzare il nome della variabile seguito dalle parentesi,
    //    esattamente come ci si comporta con il callback di una funzione normale.
    // 4. Se la funzione o l'espressione da valutare deve accettare degli argomenti questi vanno
    //    indicati tra le pipe.
    let gino = || println!("Ciao sono gino");
    //  TEST DI SCRITTURA DI UNA CLOSURE CHE ACCETTA ARGOMENTI.
    let pino = |cognome| println!("Ciao sono pino, {cognome}");
    pino("sballo");
    gino()
}
