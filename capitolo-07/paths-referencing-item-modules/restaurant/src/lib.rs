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
// Provo a riprodurre esempio a pagina 131.
//
// parte backoffice ristorante
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // dopo aver dichiarato la struct come pub. la struct diventa pubblica. Lo stesso vale per
    // toast.
    // Essendo "seasonal_fruit" privato, occorre una funzione associata per eseguire la costruzione
    // e la manipolazione dei campi della struct.
    // FUNZIONE ASSOCIATA (impl)
    impl Breakfast {
        // creo una funzione per capire quali sono i frutti stagionali (estivi).
        // toast è un puntatore al valore toast e ritorna la struct Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // dichiaro seasonal_fruit
                seasonal_fruit: String::from("Pesche"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Toast con Farina d'avena");
    // dopo l'inizializzazione di meal ho finalmente la mia colazione estiva completa.
    // meal quindi diventa una struct così composta:
    // meal {
    //  toast: "Toast con Farina d'avena",
    //  seasonal_fruit: "Pesche",
    // }
    // avendo dichiarato meal come mut posso anche eseguire un'operazione di questo tipo:
    meal.toast = String::from("Toast al profumo di chanel");
    // questa nuova associazione di una nuova stringa a meal.toast può essere vista come "un cambio
    // di idea da parte del cliente".
    // Un tentativo di modifica di "seasonal_fruit" causerebbe un errore in fase di compilazione
    // perché il campo della struct non è pubblico.
    // Per modificare seasonal_fruit è obbligatorio REINIZIALIZZARE Breakfast, chiamando summer()
    // con il suo argomento.
    // Esempio di un eventuale errore in fase di compilazione:
    // meal.seasonal_fruit = String::from("blueberries")
}
