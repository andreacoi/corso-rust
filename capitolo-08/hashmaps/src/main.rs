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
}
