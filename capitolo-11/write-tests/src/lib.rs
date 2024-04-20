#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // questa dichiarazione use serve per accedere alle porzioni di codice fuori dal modulo test,
    // che sono scritte direttamente nella root del file lib.rs
    use super::*;

    /* Scrivo un test per testare il funzionamento della funzione can can_hold.
     * La funzione larger_can_hold_smaller restituisce un assert, cioè il risultato di una verifica.
     * In particolare, restituisce ok se il test procede per come stabilito e failure se invece
     * assert non passa il test. Provando a cambiare i valori nelle due istanze della struct larger e smaller, il test restituisce FAILED.
     * Ovviamente la command line risponderà con l'elenco dei failures, cioè dei test che sono falliti.
     * */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 11,
            height: 14,
        };
        let smaller = Rectangle {
            width: 1,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }
}
