/*
* 15.3 Esempi di utilizzo del Drop Trait
* Il trait drop viene utilizzato, a differenza dei linguaggi che utilizzano i garbage collector per
* svuotare quella porzione di memoria ogni qualvolta una variabile esce fuori dallo scope.
* Questo comportamento prende il nome di drop (buttare).
* Rust infatti, non appena una variabile esce fuori dallo scope, invoca (dietro le quinte) un
* distruttore che si occupa di smaltire la porzione di heap associata a quella variabile,
* distruggendo di fatto la variabile stessa.
* Questo trait, benché sia caratteristico e idiomatico di Rust, nonché una delle sue funzionalità
* più note del linguaggio, viene trattato nel capitolo degli smart pointer perché la sua
* implementazione viene utilizzata PROPRIO nel contesto degli smart pointer.
* Ho parlato di implementazione perché tramite impl è possibile scrivere una implementazione
* dedicata del trait drop specifica per ogni smart pointer o per ogni struct di tipo Box<T>.
* Le implementazioni di drop possono variare, ma il fine ultimo rimane sempre il medesimo.
* Quello che può essere influenzato è il comportamento dell'istanza QUANDO STA PER uscire dallo
* scope.
* Scrivo un esempio dichiarando uno smart pointer custom.
* */

struct CustomSmartPointer {
    data: String,
}
// implementazione di Drop customizzata. Riporta quello che deve avvenire all'atto di eseguire il
// drop sulla struct CustomSmartPointer, in questo caso specifico stampo una stringa esplicativa.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Utilizzo drop per pulire l'heap dopo l'utilizzo della variabile {}",
            self.data
        )
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Stringa a caso 1"),
    };
    let d = CustomSmartPointer {
        data: String::from("Stringa a caso 2"),
    };
    // ho inizializzato due custom smart pointer, in modo da vedere cosa succede al loro drop.
    println!("Creati due custom Smart Pointer");
    // fine dello scope
    // Dopo questo limite dovrebbe essere invocata in maniera automatica la funzione drop che però
    // sarà l'override di quella di default utilizzata in Rust. Questo perché esiste per questi due
    // smart pointer una implementazione dedicata.
    // Output:
    // Creati due custom Smart Pointer
    // Utilizzo drop per pulire l'heap dopo l'utilizzo della variabile Stringa a caso 2
    // Utilizzo drop per pulire l'heap dopo l'utilizzo della variabile Stringa a caso 1
    //
    // Con queste implementazioni sono stato in grado di eseguire del codice arbitrario prima
    // dell'utilizzo di drop.
    // N.B. Benché sia impossibile utilizzare il drop trait in maniera volontaria da parte
    // dell'utente (viene impedito da Rust per impedire casi di double free memory) Rust mette a
    // disposizione std::mem::drop per effettuare la medesima operazione, con la differenza che il
    // metodo drop() a differenza del trait viene utilizzato semplicemente per anticipare la fine
    // del ciclo di vita di una variabile. Inoltre, è bene specificare che drop(), come metodo
    // prende possesso della variabile sul quale viene eseguito, rendendo di fatto inutilizzabile
    // la variabile dopo la chiamata.
    // Specificando ancora: È una funzione della libreria standard che prende il possesso di un valore
    // e lo fa uscire immediatamente dallo scope, chiamando così il metodo drop del trait Drop
    // se definito per quel tipo.
    // In sintesi: il trait Drop permette di definire una pulizia automatica personalizzata
    // quando un oggetto esce dal suo scope, mentre la funzione std::mem::drop offre un modo per rilasciare
    // esplicitamente una risorsa prima del tempo.
}
