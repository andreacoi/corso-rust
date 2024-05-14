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
    let y = Box::new(x);
    // ovviamente, vado anche qui a fare la comparazione:
    assert_eq!(5, *y);
    // si nota perfettamente che y utilizzata nel primo esempio è tatalmente identica a questa.
}
