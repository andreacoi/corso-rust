use std::ops::Deref;

/*
* L'operatore Deref
* Grazie all'operatore deref è possibile utilizzare gli smart pointer come puntatori regolari.
* Tutto questo è possibile farlo grazie all'operatore di deference.
*  Riporto un esempio molto semplice.
*/
fn main() {
    let x = 5;
    // adesso y è una reference che punta a x;
    let y = &x;
    // possiamo verificare che i valori sono uguali utilizzando assert_eq!
    assert_eq!(5, x);
    // scrivendo questo tipo di sintassi abbiamo detto, in sostanza:
    // verifica che IL VALORE a cui punta y sia uguale a 5. Anche in questo caso, la verifica avrà
    // successo. L'asterisco (da non confondere con l'operazione di moltiplicazione) è l'operatore
    // per eseguire l'operazione di deferenza.
    // L'operazione di deferenziazione consente di accedere al valore a cui punta un puntatore,
    // smart o grezzo che sia.
    assert_eq!(5, *y);
    // N.B. Se provassi a comparare in questo modo:
    // assert_eq!(5,y );
    // otterrei un errore in fase di compilazione perché Rust mi avvisa che non è possibile
    // comparare un numero con una reference a un numero.
    //
    // Box<T> può anche essere utilizzata come reference
    // ecco lo stesso esempio, rivisto con l'utilizzo di Box.
    // EDIT: dopo le prime prove, per esercizio ho creato un mio tipo personalizzato, chiamato
    // MyBox, che si comporta esattamente come Box e implementa il trait deref.
    let y = MyBox::new(x);
    // ovviamente, vado anche qui a fare la comparazione:
    assert_eq!(5, *y);
    // si nota perfettamente che y utilizzata nel primo esempio è tatalmente identica a questa.
    // Vi è tuttavia una differenza formale tra &x Box::new(x). Nel primo caso y punta a valore di
    // x, nel secondo esempio invece abbiamo un puntamento ad UNA COPIA di x.
    // Grazie a Box<T> possiamo creare degli smart pointer personalizzati, vediamo come fare.
    //
    // Esempio di Coercizioni di deferenziazione - vedi fn hello.
    // Rust è in grado di convertire automaticamente &MyBox in &String chiamando la funzione deref.
    let m = MyBox::new(String::from("Andrea"));
    hello(&m);
}
// 15.2.3 Definire uno smart pointer personalizzato.
// Il Box<T> è definito nella stdlib come una tupla struct con un solo elemento. Possiamo
// quindi definire una Box personalizzata allo stesso modo.
//

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// N.B. Anche se MyBox è formalmente identico a Box<T>, se andassimo a sostituire Box con MyBox
// alla riga 21 del main otterremmo un errore.
// L'errore è dovuto al fatto che Rust NON sa come deferenziare MyBox. Il trait Deref infatti è
// parte di Box<T> come specificato nella standard library. Ovviamente, essendo MyBox uno smart
// pointer personalizzato Rust non sa quali trait vengano implementati dal tipo. Questo anche se la
// struttura è identica. Per fare in modo che MyBox si comporti come Box dobbiamo implementare gli
// stessi trait. In questo caso, dobbiamo andare a creare Deref, come trait.
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Coercizioni di deferenziazione implicite
// Coercizioni di deferenziazione significa che Rust esegue automaticamente la dereferenziazione di un puntatore o di un riferimento quando necessario, ad esempio quando si cerca di accedere a un campo o si chiama un metodo su un tipo che è di fatto un riferimento. Questo rende il codice più pulito e meno verboso, poiché Rust si prende cura delle dereferenziazioni implicite al momento opportuno.
// esempio:
fn hello(name: &str) {
    println!("Ciao {name}");
}
