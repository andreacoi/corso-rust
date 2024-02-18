fn main() {
    let s1 = String::from("hello");
    // let s2 = s1 produce un errore perché si cerca di far puntare s2 a un blocco di memoria che
    // viene liberato. In aggiunta, RUST prova a liberare, alla fine dello scope, anche il blocco
    // riservato ad s2 entrando, di fatto, in un caso di double free.
    // Per "assegnare s1 a s2" si può utilizzare il metodo clone che crea una copia della memoria
    // referenziata. Questo metodo, nonostante sia funzionante, esegue codice arbitrario ed è
    // "costoso" in termini di utilizzo della memoria.
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Il discorso è nettamente diverso quando si parla di integer.
    // RUST riesce ad allocare nello stack i valori interamente i valori degli interi. Gestire gli
    // interi infatti è più semplice e, in questo modo, la copia risulta molto più veloce ed
    // intuitiva. Gli interi possono essere allocati interamente nello stack perché la loro
    // dimensione è sempre conosciuta. È comunque possibile utilizzare l'annotazione Copy sulle
    // variabili integer per forzare il compilatore ad eseguire una mera copia del valore (come in
    // clone sopra).
    // Cit. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    //
    // Metodi che supportano l'annotazione Copy:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    //
    //Es.
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    // resta comunque valido il discorso per il quale alla chiusura dello scope s1, s2, x e y
    // vengono comunque invalidati e la memoria viene consequenzialmente liberata.
    // occupo la memoria dichiarando esempio come stringa
    let esempio = String::from("esempio");
    // passo esempio all funzione prendi_possesso e in questo momento "libero" esempio dallo scope
    // di main e la porto nello scope di prendi_possesso.
    prendi_possesso(esempio);
    // Quando passo una variabile ad una funzione lo scope di quella variabile cambia per sempre.
    //    println!("{}", esempio); // questo non funziona, perché "adesso, in questo punto" esempio non è
    // nello scope di main.
    let numero = 39;
    esegui_copia(numero);

    //in questo modo stringa_a_caso viene assegnata a sac e sac e nllo scope di main
    let sac = conferisce_proprieta();

    let ultima_stringa = String::from("una stringa che andra avanti e indietro");

    let ultimo_passaggio_di_mano = dai_e_riprendi(ultima_stringa);

    //riassunto di quanto accaduto
    // esempio: drop in quanto prendi_possesso ha preso possesso della variabile
    // numero: rimane nello scope in quanto in storage nello stack (come copia) drop alla fine:
    // drop alla fine della funzione main
    // della funzione main
    // sac: entra nello scope per il callback della funzione drop alla fine della funzione main
    // ultimo_passaggio_di_mano: entrato nello scope main, drop alla fine anche lui

    // Ma è possibile passare dei valori ad una funzione senza perderne l'ownership?
    // Si, è possibile farlo utilizzando una funzione che ritorni una tupla di valori.
    let str = String::from("ultima_stringazza_finale");
    let (str2, lunghezza) = calcola_lunghezza_str(str);
    println!("La lunghezza della stringa {str2} è: {lunghezza}");
}

fn prendi_possesso(variabile: String) {
    println!("{}", variabile);
}

fn esegui_copia(numero: u32) {
    println!("{}, the 2nd", numero);
}

fn conferisce_proprieta() -> String {
    let stringa_a_caso = String::from("ciao sono una stringa a caso");
    stringa_a_caso
}
// questa funzione prende stringa in suo possesso e la restituisce nuovamente alla funzione
// chiamante (main)
fn dai_e_riprendi(stringa: String) -> String {
    stringa
}

fn calcola_lunghezza_str(stringa: String) -> (String, usize) {
    let lunghezza = stringa.len();
    (stringa, lunghezza)
}
