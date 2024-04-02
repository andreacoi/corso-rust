/*
* 10.3 I trait: definizione di comportamenti condivisi
* I trait in Rust servono per definire delle funzionalità che un tipo può avere.
* Possono essere ricondotti a degli elementi presenti in altri linguaggi chiamati interfacce,
* seppur con sostanziali differenze.
*
* 10.3.1 Definzione di un trait
* Il comportamento di un tipo consiste in dei metodi che possiamo chiamare su quel tipo. Differenti
* tipi possono condividere lo stesso comportamento se possiamo chiamare gli stessi metodi su di
* essi. Le definizioni di un trait sono un modo per raggruppare le signature dei metodi e creare un
* set di comportamenti necessari per eseguire determinati task.
* Un trait può essere dichiarato utilizzando la parola chiave trait (anche preceduta da pub se
* necessario) e seguita dal nome del trait, con le parentesi graffe.
* es.
* */
pub trait Sommario {
    fn summarize(&self) -> String;
}
/*
* All'interno delle parentesi graffe bisogna indicare le signature delle funzioni incluse in quel
* trait. La funzione termina con un punto e virgola perché l'implementazione OBBLIGATORIA sarà a
* carico del tipo che implementerà il trait.
* In questo modo, tutti i tipi che dovessero implementare Sommario saranno obbligati a specificare
* il comportamento della funzione summarize, che può differire ogni volta.
* Quest'obbligo verrà imposto dal compilatore.

* 10.3.2 Implementare un trait su un tipo
* Proviamo ad applicare il trait Sommario a due tipi, uno è una struct chiamata News, con i campi:
* headline, location, author, content, l'altro è una struct chiamata Tweet con i campi: username,
* content, reply, retweet.
*es.
**/

// News è un nuovo tipo su cui poter implementare dei traits.
pub struct News {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// Lo stesso discorso vale per Tweet, anche qui è possibile implementare dei trait.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// scrivo il trait per news, se non dichiaro la funzione summarize, rust mi restituisce errore.
impl Sommario for News {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// scrivo il trait per Tweet, anche qui sono obbligato a dichiarare la funzione summarize per non
// ottenere errori.
impl Sommario for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
/*
* Il testo riporta questi trait e le loro implementazioni in un file lib.rs e poi li include
* tramite il crate aggregator, in questo modo: use aggregator::{Summary, Tweet}.
* Avendo lavorato solo nel main non è necessario utilizzare use.*/

/*
* 10.3.3 Implementazioni di default
* Nonostante sia obbligatorio dichiarare una funzionalità per un metodo quando si va ad utilizzare
* un'implementazione in un trait, potrebbe succedere di voler dichiarare una funzionalità di
* default per quel metodo. In tal caso è possibile specificare la funzionalità del metodo
* direttamente nel trait.
* Così come le funzioni normali anche i metodi supportano l'override. Inoltre, con le
* implementazioni di default è possibile scrivere meno codice.
* Es.
*   pub trait Sommario {
*   fn summarize(&self) -> String {
*     String::from("Leggi tutto");
*     // implementazione di default
*     // TODO: OVERRIDE ED ESTENSIONE ALL'IMPLEMENTAZIONE DI DEFAULT.
*   }
}
* */
fn main() {
    /* Utilizzo dei trait e delle loro implementazioni
     * */
    let tweet = Tweet {
        username: String::from("andrea.coi"),
        content: String::from("Il mio primo Tweet"),
        reply: false,
        retweet: false,
    };
    println!("Il tuo primo tweet: {}", tweet.summarize());
}
