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
    // Avendo creato l'iteratore posso usare un ciclo for per scorrere i suoi elementi e gestire il
    // suo termine e i vari step.
    // Es.
    for val in v1_iter {
        // ATTENZIONE - ad ogni giro del ciclo for, l'iteratore viene decrementato di 1, quindi si
        // avvicina alla fine dei cicli.
        println!("Elemento: {}", val);
        // ATTENZIONE - il ciclo for non consuma l'iteratore perché dietro le quinte il loop rende
        // mut l'elemento v1_iter.
        // Nel file src/lib.rs ho inserito un esempio di metodo che consuma un iteratore.
    }
    // 13.2.3 Metodi che producono altri iteratori
    // Il metodo map() produce un altro iteratore, a partire da un iteratore iniziale. Si dice che
    // metodi come map() cambiano alcuni aspetti dell'iteratore iniziale, senza consumarlo.
    // Ecco un esempio inutile, perché non produce nessun output.
    let v2 = vec![3, 4, 5];
    // questa specifica sintassi rende inutile il codice perché in Rust gli iteratori sono "pigri"
    // e questo codice non viene mai utilizzato.
    let v3 = v2.iter().map(|x| x + 1);
}
