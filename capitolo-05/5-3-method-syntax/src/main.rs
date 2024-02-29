// Metodi e sintassi dei Metodi
// I metodi sono esattamente identici alle funzioni eccezion fatta per due particolari:
// - Sono validi nello scope di una struct;
// - il primo parametro è SEMPRE una self;
// Per dichiarare un metodo e far capire a Rust che si tratta di un metodo nel contesto di QUELLA
// STRUCT bisogna utilizzare la parola chiave impl seguita dal nome della struct.

#[derive(Debug)]
struct Rettangolo {
    width: u32,
    height: u32,
}
// serie di metodi che riguardano il contesto Rettangolo
impl Rettangolo {
    // ecco il self come primo parametro:
    // sostanzialmente dice: usa ciò che trovi nella struct, agisci con la struct - vedi metodo di
    // accesso ai campi: self.nomecampo
    // È OBBLIGATORIO UTLIZZARE il parametro self del tipo Self come PRIMO PARAMETRO DURANTE LA
    // COSTRUZIONE DI UN METODO. Rust ce lo lascia abbreviare in self.
    // N.B. anche in questo caso attuiamo una procedura di borrow per il self. Infatti prendiamo in
    // prestito i valori della struct per il calcolo dell'area del rettangolo. Quindi, quando
    // inseriamo il parametro self lo facciamo così: &self.

    fn area(&self) -> u32 {
        self.width * self.height
    }
    // funzioni associate
    // Tutte le funzioni che sono all'interno di un impl si definiscono funzioni associate. Un
    // esempio già visto è String::from(). Esistono, tuttavia, funzioni associate che non hanno
    // self come primo parametro perché non hanno bisogno di un'istanza del proprio tipo (nel
    // nostro caso Rettangolo) su cui lavorare.
    // Es.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // questa funzione ci consente ad esempio di generare un "rettangolo quadrato" e non
    // ci obbliga a specificare lo stesso valore più volte negli argomenti.
    // Viene chiamato Self perché questa è una funzione che fa da costruttore dell'oggetto
    // Rettangolo. Le funzioni associate, a differenza dei metodi, vengono utilizzate spesso come
    // costruttori.
}

fn main() {
    let rettangolo1 = Rettangolo {
        width: 50,
        height: 30,
    };

    // area è un METODO della struct RETTANGOLO, gli argomenti sono i campi di rettangolo1.
    println!("L'area del tuo rettangolo è: {}", rettangolo1.area());
}
