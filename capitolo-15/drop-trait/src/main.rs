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
    println!("Hello, world!");
}
