fn main() {
    let stringa =
        String::from("Nel mezzo del cammin di nostra vita, mi ritrovai per una selva oscura.");
    let lunghezza = calcola_lunghezza(&stringa);
    println!("La lunghezza del versetto: {} è {}", stringa, lunghezza);
}

fn calcola_lunghezza(stringa: &String) -> usize {
    let lunghezza = stringa.len();
    lunghezza
}
