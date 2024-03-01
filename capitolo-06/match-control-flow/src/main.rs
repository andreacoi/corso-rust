// Il costrutto match è uno dei più potenti costrutti di RUST per il controllo del flusso.
// Permette di confrontare dei valori con una serie di pattern.
//
// I pattern possono essere composti da: letterali, stringhe, nomi_variabili, wildcard e altro.
// Funziona essenzialmente come i contatori di monete: quando si inserisce una moneta in un
// contatore di monete, lui valuta la moneta sulla base della dimensione della stessa. Quando la
// moneta arriva al buco che "matcha" la sua dimensione cade all'interno e viene conteggiata,
// altrimenti no.
//
// Questo programma fa esattamente questo: il contatore di monete:
//

enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// è possibile abbinare dei valori a dei pattern che vanno in match. In questo caso ad esempio, ho
// creato una enum con degli stati americani. Tra tutte le monete in uso negli Stati Uniti solo il
// Minnesota continua ad utilizzare un quarto di dollaro particolare. È possibile quindi capire
// quando siamo davanti a quel quarto di dollaro se aggiungiamo questo valore extra al pattern di
// confronto.
#[derive(Debug)]
enum UsState {
    Alabama,
    Minnesota,
}
// la funzione valore in centesimi ritorna un intero unsigned a 8 bit
// argomento accettato: una enum Moneta --> coin
// Sulla base della Enum passata alla funzione, il codice inizia ad entrare nei vari bracket di
// match: es. se coin è uguale a Moneta::Penny la funzione ritorna 1, se coin è uguale a
// Moneta::Nickel ritorna 5...
fn valore_in_centesimi(coin: Moneta) -> u8 {
    match coin {
        Moneta::Penny => 1,
        Moneta::Nickel => 5,
        Moneta::Dime => 10,
        Moneta::Quarter(state) => {
            println!("Oh! Questo quarto di dollaro raro viene da: {:?}", state);
            25
        }
    }
}

// Match con Option<T>
// è possibile creare una funzione che verifichi che esista un valore in una variabile Option<T> ed
// esegua delle operazioni sulla base dell'esistenza o della non esistenza della Option.
// Es. questa funzione aggiunge uno all'argomento, se questo ESISTE, altrimenti ritorna None.

fn aggiungi_uno(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = aggiungi_uno(five);
    println!("Valore di six: {:?}", six);
}

// Nota per Option<T> per capirla meglio:
// Come idea è simile a una promise: mi aspetto un valore di un certo tipo, a me ignoto. Se quel
// valore esiste comportati in una certa maniera --> Option<tipo_valore>, altrimenti in un'altra --> None.
//
// I match in RUST sono ESAUSTIVI:
// BISOGNA PREVEDERE TUTTE LE POSSIBILITÀ OFFFERTE PER RENDERE IL CODICE VALIDO. Fortunatamente RUST ci avvisa e sa come comportarsi in fase di compilazione.
// Questa regola è ancor più valida in casi come Option<T>, DOVE NON POSSIAMO ASSOLUTAMENTE LASCIARE "APERTA" LA POSSIBILITÀ DI RISCONTRARE UN "NONE" non previsto.
