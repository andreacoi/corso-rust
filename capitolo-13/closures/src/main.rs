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
fn main() {}
