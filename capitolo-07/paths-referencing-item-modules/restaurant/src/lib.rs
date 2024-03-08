// Per poter utilizzare i moduli dichiarati nelle librerie ci si può riferire ad essi come quando
// si esplorano le cartelle di un filesystem.
// Per questo motivo i path di un modulo si presentano in due forme:
// - path assoluti: partono dalla root del crate utilizzando la parola chiave crate. Dopo ogni
// livello, la separazione degli stessi avviene utilizzando i due punti ripetuti due volte: "(::)"
// vedi r. 21
// - path relativi: partono dal modulo corrente ed utilizzano keyword come super o self per
// riferirsi al modulo corrente.
//

mod frontend_ristorante {
    mod accoglienza {
        fn accogli_clientela() {}
    }
}

fn valida_prenotazione() {}

mod backend_ristorante {

    pub fn mangia_al_ristorante() {
        // esempio di path assoluto
        crate::frontend_ristorante::accoglienza::accogli_clientela();
        // esempio di path relativo
        frontend_ristorante::accoglienza::accogli_clientela();

        // La parola chiave SUPER e il suo utilizzo nei path relativi.
        // super funziona come "cd ..", consente di far riferimento ad un elemento in un "modulo padre"
        // Es. Se avessi bisogno di accedere, da qui, alla funzione valida_prenotazione()
        // potrei farlo in questo modo:
        //
        super::valida_prenotazione()
    }
}

// In Rust tutti gli elementi sono private nei confronti dei loro genitori. L'elemento child,
// invece può sempre accedere al suo elemento parent.
// Attenzione però! RENDERE UN MODULO PUBBLICO NON NE RENDE PUBBLICI I SUOI ELEMENTI!
//
