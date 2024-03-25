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
    let mut largest = &number_list[0];
    // inizio il ciclo for per ciclare tra i numeri del vettore.
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Il numero più grande dell'insieme è: {}", largest);
}
