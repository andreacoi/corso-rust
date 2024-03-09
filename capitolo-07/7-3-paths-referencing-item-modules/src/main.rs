// per gli esempi di questoc capitolo devi andare su restaurant/src/lib.rs
// rendere pubbliche struct e enum
// applicando la keyword durante la dichiarazione di una struct, questa diventerà public, ma i suoi
// campi resteranno private, salvo che non sia diversamente specificato.
fn main() {
    pub struct Colazione {
        italia: String,
        internazionale: String,
    }
    // in questo specifico caso, la struct Colazione è pubblica, ma i suoi campi Italia e
    // internazionale RESTANO COMUNQUE PRIVATI.
    // Es. con campi pubblici
    pub struct Pranzo {
        pub jap: String,
        pub chn: String,
    }
    // il discorso, invece, è meno stringente quando parliamo di enum.
    // Con le enum infatti è sufficiente dichiarare come pubblica la enum stessa per renderne
    // pubbliche tutte le sue varianti.
    // Es.
    pub enum Allergeni {
        Salsa,
        Insalata,
    }
    // per quanto riguarda le struct con privacy mista: un campo pub e uno private, avremo sempre
    // bisogno di una funzione pubblica che ci aiuti ad effettuare il construct della struct.
    // Vedi esempio 7.9 - TODO: PROVA A REPLICARLO NEI MODULI, QUANDO SEI PIÙ LUCIDO
}
