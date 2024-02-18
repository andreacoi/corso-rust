fn main() {
    let mut x = 5;
    // le costanti sono sempre immutabili, quando le si dichiara va sempre dichiarato il tipo (in
    // questo caso u32. Il naming per le costanti segue delle best practices (tutto maiuscolo, con
    // le diverse parole separate da un underscore))
    const PI: u32 = 314;
    println!("The value of x: {x}");
    x = 6;
    println!("The value of x: {x}");
    println!("Il valore di pi è: {PI}");

    // shadowing - caratterstica del linguaggio per la quale è possibile ridichiarare una variabile
    // con lo stesso nome andando di fatto a sostituirne il valore.
    //

    let x = 5;
    let x = x + 1;
    {
        // in questo caso specifico, la ridichiarazione riguarda lo scope (!), cioè quanto
        // racchiuso tra le graffe
        let x = x * 2;
        println!("Il valore di x è: {x}");
    }
    println!("Qui fuori invece il valore di x è: {x}");

    // lo shadowing, per definizione, permette di riutilizzare la stessa variabile, lo stesso nome
    // per riassegnare un valore completamente nuovo. La differenza con il "mut" è che con lo
    // shadowing in un sol colpo può cambiare anche il tipo. L'esempio qui in basso calza a
    // pennello: spaces passa dall'essere una stringa all'essere un intero.
    let spaces = "  ";
    let spaces = spaces.len();

    println!("spaces è: {spaces}");
}
