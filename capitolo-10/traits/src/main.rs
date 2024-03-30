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
* */
fn main() {
    println!("Hello, world!");
}
