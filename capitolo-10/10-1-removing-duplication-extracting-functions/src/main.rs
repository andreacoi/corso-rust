// 10.1 Rimuovere la duplicazione "estraendo" una funzione
//  I generics ci permettono di sostituire tipi specifici con un "segnaposto" che rappresenti tipi
//  multipli per rimuovere la duplicazione del codice.
//  Nella funzione, andando per gradi vedremo come rimuovere la duplicazione in un modo che non
//  coinvolge i tipi generici estraendo una funzione che rimpiazza valori specifici con un "segnaposto" che rappresenti valori multipli.
fn main() {
    // Funzione per stabilire il più grande tra i numeri di un insieme.
    // Dichiariamo un vettore con i numeri da confrontare.
    let number_list = vec![34, 50, 25, 100, 65];
    // dichiaro largest come number_list[0], in modo da avere un punto da cui partire per il
    // confronto. Lo rendo mut perché poi verrà rimpiazzato con un nuovo valore qualora nel ciclo
    // ci si imbattesse nel numero più grande.
    /* let mut largest = &number_list[0];
    // inizio il ciclo for per ciclare tra i numeri del vettore.
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    Una volta creata la funzione lascio a lei il compito di calcolare il più grande tra i numeri alleggerendo il main.
    */
    let result = largest(&number_list);
    println!("Il numero più grande dell'insieme è: {}", result);
    // Viene abbastanza ovvio che, se dovessimo riutilizzare questa porzione di codice per
    // calcolare il più grande in un insieme di numeri dovremmo reinizializzare il ciclo for, fare
    // il check dei valori con un altro if. Questo è dispendioso sia in termini di righe di codice,
    // sia in termini di debug, perché ci troveremmo a fare il debug della stessa funzione in punti
    // diversi del codice.
    // Fortunatamente, come in tanti altri linguaggi, Rust ci permette di "estrarre delle funzioni"
    // per riutilizzarle in futuro, che è proprio il principio fondante dei generics.
}

// Estraggo la funzione che ho utilizzato per calcolare il più grande in un insieme di numeri.
fn largest(list: &[u32]) -> &u32 {
    // inizializza largest al primo elemento di list
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    // ritorno largest come variabile.
    largest
}
