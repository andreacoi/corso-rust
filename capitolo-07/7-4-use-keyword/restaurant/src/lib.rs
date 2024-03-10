// inizializza il modulo per gestire il frontend del ristorante.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_whitelist() {}
    }
}

// la keyword use consente di creare "un symlink" al modulo per consentire di utilizzare le
// funzioni in esso contenuto senza dichiarare ogni volta il path completo.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // chiamata valida e possibile proprio in virtù della keyword use.
    // così come per i path, anche con use, Rust esegue il check della privacy, così come per gli
    // altri path.
    // N.B. Attenzione! È possibile utilizzare i crates di cui abbiamo creato i collegamenti con la
    // keyword use SOLO NELLO SCOPE in cui lo use statement è stato specificato.
    // Ad esempio, se racchiudessi la funzione eat_at_restaurant in un modulo, riceverei un errore
    // di compilazione dal compilatore.
    hosting::add_to_whitelist();
}

