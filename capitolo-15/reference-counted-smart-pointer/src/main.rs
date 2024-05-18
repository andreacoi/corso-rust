/*
* 15.4 Rc<T> Oggetti a conteggio di riferimenti
* Un contatore di riferimenti è necessario nel caso in cui la proprietà di un valore sia detenuta
* da piàù proprietari contemporaneamente.
* Il libro di Rust fa un ottimo esempio di un caso in cui potrebbe essere utile avere un contatore.
* Immaginiamo di avere una stanza con una TV
*
     ______________
    |   TV        |
    |_____________|
       /      \
      /        \
    /            \
   /              \
 /__________________\
/                    \

 1\  O   O   O   O   O  /5
    \|   |   |   |   | /
     |   |   |   |   |
    /2   3   4   5  /
   /               /
  /_______________/
*
* Entra la persona 1 nella stanza e trova la TV spenta. La accende e inizia a guardarla. In questo
* momento Persona 1 è il proprietario del valore TV. Dopo un po' arriva anche Persona 2 e inizia
* a guardare la TV già accesa. Lo stesso discorso vale per 3, 4 e 5. In questo momento la scenetta
* è completa. I 5 attori condividono "la visione" della TV, sono diventati parimenti proprietari
* del "valore" TV. Essendo proprietari allo stesso modo del valore TV, per definizione di proprietà
* in Rust, qualora uno di questi decidesse di aver finito di guardare la TV e la spegnesse, farebbe
* arrabbiare tantissimo tutti gli altri spettatori che non avevano ancora deciso di smettere di
* guardarla.
* Immaginando scene come questa ci viene in mente uno dei casi in cui utilizzare Rc<T>.
* Rc<T> è una struttura dati che permette di conteggiare quanti riferimenti sono presenti ad un
* valore in un determinato scope.
* Utilizzando Rc<T> possiamo sapere quanti spettatori stiano ancora guardando la TV e consentirne
* lo spegnimento solo quando anche l'ultimo spettatore (proprietario) sia uscito dalla stanza.
* È chiaro che questo costrutto si utilizza solo nel caso in cui le regole riguardanti l'ownership
* non possono essere stabilite in fase di compilazione dal borrow checker.
* Un esempio di un caso fattuale può essere fatto con una Cons List.
* Rc, come struttura dati va indicata nel prelude.
* Riporterò prima un esempio non funzionante e poi lo correggerò con la versione OK, utilizzando
* Rc<T>.
*/

fn main() {
    println!("Hello, world!");
}
