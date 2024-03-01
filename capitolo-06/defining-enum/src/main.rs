// struttura base di una enum:
// a differenza di una struct questa struttura dati indica le VARIANTI che può assumere IpAddrKind.
// Entrambe le varianti sono fondamentali, ma un IP può essere di tipo V4 OPPURE V6. Difatti non
// esistono IP che siano ALLO STESSO TEMPO sia V4 che V6.
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // Quit non ha dati associati
    Quit,
    // Move ha dei dati associati con dei campi con nomi stabiliti (esattamente come le struct).
    Move { x: i32, y: i32 },
    // Write include una stringa singola
    Write(String),
    // ChangeColor ha tre valori, tutti i32
    ChangeColor(i32, i32, i32),
}
// così come le struct anche le enum possono utilizzare impl per i propri metodi
impl Message {
    fn call(&self) {}
}
// OPTION ENUM
// I tipi OPTION codificano uno scenario molto comune durante le fasi di sviluppo, dove, ad
// esempio, un valore PUÒ ESSERE qualcosa o PUÒ NON ESSERE NULLA.
// Option<T> è viene utilizzato per gestire le eccezioni derivanti dal fatto che un valore può non
// esistere. Quando un valore viene dichiarato come Option<T> può succedere che non esista (null),
// diversamente RUST impone di dichiararne il tipo. Specificando, quando si dichiara il tipo di una
// variabile dev'essere GARANTITO che questa esisterà sempre per tutto lo scope. Questo non fa che
// incrementare la sicurezza del codice RUST.

// possiamo anche definire una funzione che ha come argomento un tipo IpAddrKind
// Es.
fn route(ip_kind: IpAddrKind) {}

fn main() {
    // grazie alla enum IPAddr (che è comunque presente nella standard Library), ho potuto
    // dichiarare un indirizzo IP Versione 4 senza creare enum nidificate in struct con:
    // - tipo,
    // - indirizzo...
    let home = IpAddr::V4(127, 0, 0, 1);
}

