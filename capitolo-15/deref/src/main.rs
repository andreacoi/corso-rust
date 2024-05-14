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
}
