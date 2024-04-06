use std::fmt::Display;

/* Lifetime - tempo di vita degli oggetti in Rust.
* In Rust, ogni reference ha un Lifetime. Il Lifetime è lo scope per il quale quella determinata
* reference è valida. Così come per i tipi anche le Lifetime possono essere implicite o subire
* inferenza.
* */
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {r}");

    let string1 = String::from("long long maaan!");
    let string2 = String::from("short");
    // mi sposto, come scope nelle parentesi graffe.
    {
        let string2 = String::from("short");
        /* La firma della funzione longest <'a> specifica che la slice della stringa ritornata
         * dalla funzione stessa deve avere una lifetime almeno pari a quella di 'a.
         *  Specificando meglio, IL LIFETIME DELLA REFERENCE RITORNATA DALLA FUNZIONE LONGEST
         *  COINCIDE CON IL LIFETIME PIÙ PICCOLO TRA I LIFETIME DEI VALORI A CUI SI RIFERISCONO GLI
         *  ARGOMENTI DELLA FUNZIONE.
         *  È bene specificare che la funzione longest "non sa" quanto vivranno x e y. Quello "che
         *  sa" specificando il lifetime è che "dovrà sostituire" lo scope con 'a per soddisfare il
         *  requisito richiesto dalla firma della funzione stessa.
         */
        let result = longest(string1.as_str(), string2.as_str());
    }
    println!("Test {}", lnv2(&string1, &string2, 5));
}

// 10.4.5 Annotazioni dei lifetime nelle funzioni
// Le Annotazioni non hanno un grande significato di per sé. Servono solo ad indicare che la
// reference ritornata dalla funzione è valida finché ENTRAMBI i parametri della funzione stessa
// sono valide.
// L'annotazione si scrive tra virgolette angolari dopo la dichiarazione della funzione, come i
// generics utilizzando l'apostrofo e la definizione più breve possibile (solitamente a, b, c...).
// Es.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 10.5 Unisco tutti i concetti in un'unica funzione
fn lnv2<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annunciaziò {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
