fn main() {
    // I Vettori
    // Un vettore è una struttura dati che permette di memorizzare dati dello stesso tipo in
    // maniera consequenziale. Per creare un vettore si utilizza la seguente sintassi:
    let v: Vec<i32> = Vec::new();
    // Questo è il modo pià corretto, perchè è ben scritto, riporta il tipo degli elementi che vi
    // saranno contenuti. Tuttavia è possibile utilizzare la macro vec! per inizializzare un
    // vettore per inferenza (deduzione del tipo a carico del compilatore derivanti dai valori
    // dichiarati nel vettore). In questo caso, non avendo specificato dei valori nel vettore
    // l'inferenza è impossibile.
    // Inizializzazione vettore per inferenza
    let v1 = vec![1, 2, 3];
    // La sintassi è molto più stringata e semplice. Specificando come valori 1,2,3 Rust riesce a
    // capire che ci troviamo davanti a un vettore probabilmente <i32>.
    // Altre note
    // Nelle virgolette a spina di pesce possiamo specificare qualsiasi tipo.
    //
    // # Aggiornare un vettore
    // Per aggiornare un vettore è possibile utilizzare il metodo push. Anche per il metodo push
    // vale la regola dell'inferenza e anche i vettori devono essere dichiarati mut per poter
    // essere variati. Esempio di creazione e aggiornamento di un vettore.
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    // # Leggere elementi di un vettore
    let v3: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Per leggere gli elementi di un vettore sono disponibili due strade:
    // - il metodo .get
    // - gli indici del vettore stesso.

    // Easy: Accedo all'indice 2 del reference a v3 - borrow
    let terzo_elemento: &i32 = &v3[2];
    println!(
        "Modo Easy: Il terzo elemento dell'array v3 è: {}",
        terzo_elemento
    );
    // Hard: Uso una option per tutelarmi dagli errori o da un indice non esiste, associo quindi
    // v3.get(2) a trd.
    let trd: Option<&i32> = v3.get(2);
    match trd {
        Some(trd) => println!("Modo pro: il terzo elemento è sempre: {}", trd),
        None => println!("Non esiste alcun terzo elemento!"),
    }

    // Il risultato del metodo .get è sempre una Option<T>, per cui possiamo chiamare match per
    // gestire eventuali errori derivanti da un index out of range.
    // Proviamo a spaccare il programma:
    //  let v_non_esistente_easy = &v3[1000]; !PANIC
    //  let v_non_esistente_hard = v.get(100); -> RETURN NONE WITHOUT PANICKING
    //  v_non_esistente_easy fa andare in panico il programma perché NON esiste l'indice 1000.
    //  Quando viene utilizzato il metodo .get, se l'indice non esiste questo ritorna NONE senza
    //  causare il panic del programma.
    //
    //  # Iterare nei valori di un vettore
    //  È possibile iterare nei valori di un vettore utilizzando il classico ciclo for
    //  referenziando il vettore stesso.
    for i in &v3 {
        println!("{}", i);
    }
    //
    // # Usare una enum per immagazzinare dati di tipi multipli
    // I vettori, per definizione, possono ospitare un solo tipo. Tuttavia, in alcuni casi,
    // potrebbe rendersi necessario stilare una lista di elementi di tipi diversi. In questi casi
    // possiamo utilizzare le enum.
    // Caso concreto
    // Abbiamo una enum che rappresenta una cella di un foglio di calcolo che può contenere:
    // - String
    // - Float
    // - Integer
    enum CellaFoglioCalcolo {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        CellaFoglioCalcolo::Int(10),
        CellaFoglioCalcolo::Float(8.7),
        CellaFoglioCalcolo::Text(String::from("Hello world")),
    ];
    // Utilizzando una enum combinata con un match garantisce che il codice non riporti condizioni
    // inattese, nonostante "si lavori" sull'heap. Ça va sans dire che che nel momento in cui NON
    // si dovessero conoscere TUTTI i tipi che il programma ritornerà a runtime, la "tecnica" enum
    // non funzionerà. Come al solito, RUST CERCA LA SICUREZZA A TUTTI I COSTI.
    //
    // # Rimuovere un vettore, rimuoverà i suoi elementi
    // Come per ogni altra struttura in rust quando un vettore abbandona lo scope viene cancellato
    // e con esso anche i suoi elementi.
    // Esempio
    {
        let vettore = vec![0, 1, 2];
        // vettore esce dallo scope qui, viene distrutto e con esso i suoi valori.
    }
}
