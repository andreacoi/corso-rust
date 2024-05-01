/* 13.2 Gli iteratori
Gli iteratori sono un pattern mutuato dalla programmazione funzionale che consentono di eseguire dei task su una sequenza di elementi.
Se non ci fossero gli iteratori sarebbe necessario creare una funzione in grado di capire il TOTALE degli elementi iterabili, stabilire quando incrementare il conteggio, stabilire quando terminarlo.
Per via della natura "pigra" di questo costrutto in Rust, le iterazioni di ogni elemento vengono consumate all'utilizzo delle stesse.
*/
fn main() {
    // un vettore è il clossico esempio di un elemento che può essere iterato.
    let v1 = vec![1, 2, 3, 4];
    // con il metodo iter applicato su v1 si crea un iteratore.
    // N.B. Al momento in cui viene creato, l'iteratore NON consuma cicli.
    let v1_iter = v1.iter();
}
