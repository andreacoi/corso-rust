// Usa RefCell<T> quando hai bisogno di mutare dati che si trovano all'interno di una struttura che è altrimenti immutabile.
// È utile per bypassare le regole di mutabilità di Rust in modo sicuro.
// La differenza con Box<T> è che i dati contenuti in Box<T> non sono mutabili, a differenza dei
// dati contenuti in RefCell. Box<T> inoltre verifica la mutabilità a compile time, RefCell, invece
// lo fa a runtime.
// TODO: PROVARE A SCRIVERE LIBRERIA PER CHIAMATE A SERVIZIO API PER LA GESTIONE DEI MESSAGGI PAG.
// 337.
fn main() {
    println!("Hello, world!");
}
