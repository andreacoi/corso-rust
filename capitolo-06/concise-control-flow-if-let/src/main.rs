// la sintassi if let permette di combinare if e let per ottenere un risultato uguale a un match
// con due sole brackets.

fn main() {
    /*
    let config_max = Some(10u8);
    match config_max {
        Some(max) => print!("{}", max),
        _ => (),
    }
    Questo codice, avendo due sole condizioni può essere scritto, risparmiando linee di codice con il costrutto if let.
    */
    let config_max = Some(4u8);
    if let Some(max) = config_max {
        print!("Il massimo è {}", max);
    }
}
