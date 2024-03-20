use std::collections::HashMap;

fn main() {
    // # Gli hash maps
    // Gli hash maps sono come come gli array associativi. Servono per effettuare un'associazione
    // chiave valore diversa da come la si fa con i vettori (usando gli indici).
    // Servono per dichiarare una chiave K di qualsiasi tipo.
    // Esempio: pensa ai punteggi di due squadre. Squadra blu: 10 punti - Squadra gialla: 20 punti.
    // ## 8.3.1 Creare un hashmap
    // Per creare un hashmap bisogna innanzitutto dichiarare un import con use:
    // use std::collections::HashMap
    let mut scores = HashMap::new();
    // aggiorna punteggi
    scores.insert(String::from("Squadra Blu"), 10);
    scores.insert(String::from("Squadra Gialla"), 30);
    // 8.3.2 Accedere ai valori contenuti in una hashmap
    // Per accedere ai valori contenuti in una hashmap è necessario utilizzare il metodo get che
    // restituisce un Option con il valore associato alla chiave se la chiave esiste, e None se la
    // chiave non esiste.
    let nome_squadra = String::from("Squadra Blu");
    let punteggio_s_blu = scores.get(&nome_squadra).copied().unwrap_or(0);
    // .copied converte la Option<&i32> in una Option<i32>, successivamente, unwrap_or gestisce un
    // possibile valore None settandolo a 0.
    println!("Punteggio squadra Blu: {}", punteggio_s_blu);

    // Accedere ai valori di una hashmap utilizzando un ciclo for
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 8.3.3 HashMap e Ownership
    // Per i tipi che implementano il metodo Copy (vedi i32) i valori vengono copiati nella
    // hashmap, le stringhe invece vengono SPOSTATE nella hashmap e la proprietà cambia. La stringa
    // diventa di proprietà della hashmap.
    //
    let nome_campo = String::from("Colore preferito");
    let valore = String::from("Rosso");

    let mut hm = HashMap::new();
    // nella riga successiva inserisco nome_campo e valore nella hashmap.
    // DA QUESTO MOMENTO LA PROPRIETÀ di nome_campo e di valore CAMBIA.
    hm.insert(nome_campo, valore);

    // 8.3.4 Aggiornare una hashmap
    // 8.3.4.1 Sovrascrivere un valore
    // Per sovrascrivere un valore in una hashmap è sufficiente chiamare insert su hashmap (in
    // questo caso scores) specificando la chiave identica ad una già esistente (in questo caso
    // abbiamo aggiornato il punteggio della squadra blu).
    scores.insert(String::from("Squadra Blu"), 70);
    let punteggio = scores
        .get(&String::from("Squadra Blu"))
        .copied()
        .unwrap_or(0);
    println!("==== Overwrite valore punteggio Squadra Blu ====");
    println!("Punteggio aggiornato: Squadra Blu --> {punteggio}");

    // 8.3.4.2 Aggiungere una chiave e un valore se questi non esistono
    // Il metodo entry consente di verificare che una chiave esista. Aggiungendo or_insert a entry,
    // questo si comporterà in due modi possibili:
    // 1. tornerà una reference mut al valore della chiave verificata da entry (se la chiave esiste)
    // 2. se la chiave non esiste, procederà alla creazione della stessa e ritornerà (anche qui)
    //    una reference mut al valore della nuova chiave.
    // Test
    scores.entry(String::from("Squadra Blu")).or_insert(90);
    println!("==== Test del metodo entry e di on_insert ====");
    println!("{:?}", scores);
    println!("==== Fine del test con entry esistente, nuovo test... ====");
    scores.entry(String::from("Squadra Viola")).or_insert(90);
    println!("{:?}", scores);

    // 8.3.4.3 Aggiornare un valore sulla base del vecchio valore
    // Può capitare di dover aggiornare un valore in una hashmap su base condizionale.
    //
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // se map.entry(word) esiste count al valore referenziato di word, altrimenti il valore di
        // word è 0 (sempre referenziato).
        let count = map.entry(word).or_insert(0);
        // entry ritorna una reference, per cui per aggiornare count dobbiamo deferenziarlo.
        *count += 1;
    }
    println!("{:?}", map);
}
