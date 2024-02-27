fn main() {
    // dichiarando la struct si parte PRIMA a dichiarare i tipi di campo contenuti nella struct
    // questo processo si chiama DEFINIZIONE DELLA STRUCT.
    struct User {
        active: bool,
        username: String,
        email: String,
        password: String,
    }

    let user1 = User {
        username: String::from("Andrea"),
        email: String::from("abc@example.com"),
        password: String::from("password"),
        active: true,
    };
    // quando creiamo un'istanza della struct non siamo obbligati a rispettare l'ordine dei campi
    // nel modo in cui li abbiamo dichiarati. Dobbiamo, però, specificare un valore per ogni campo
    // dichiarato nella struct.
    // È possibile accedere ai campi utilizzando la dot notation.

    let mut user2 = User {
        username: String::from("Gino"),
        email: String::from("andrea@example.com"),
        password: String::from("password"),
        active: false,
    };
    //  è possibile variare il valore di un campo utilizzando la dot notation (SOLO SE L'ISTANZA
    //  VIENE DICHIARATA COME MUT). ATTENZIONE! NON È POSSIBILE DICHIARARE UN SINGOLO CAMPO COME
    //  MUT.
    user2.active = true;

    // è possibile dichiarare una funzione che faccia da costruttore per una struct e che gli
    // argomenti stessi della funzione siano passati come valori dei fields della struct stessa.
    // SHORTHAND STRUCT INITIALIZATION
    // Se uno degli argomenti della funzione ha lo stesso nome di uno dei campi della struct è
    // possibile scrivere l'associazione dei valori dell'istanza con la sola etichetta
    // Es. active: true, username, email, password = "gino"
    fn crea_utente(username: String, email: String) -> User {
        User {
            active: true,
            //SHORTHAND INITIALIZATION
            username,
            //SHORTHAND INITIALIZATION
            email,
            password: String::from("Password"),
        }
    }
    let user3 = crea_utente(String::from("Pino"), String::from("email"));

    // Utilizzando la sintassi STRUCT UPDATE è possibile prendere i valori da un'altra struct già
    // istanziata.
    //
    let user4 = User {
        active: user2.active,
        username: user1.username,
        email: String::from("mysecretmail@mail.com"),
        password: String::from("password"),
    };
    // è possibile scrivere user5 in questo modo (se il campo da variare è solo uno)

    let user5 = User {
        email: String::from("mymail"),
        // sintassi per copiare i gli altri valori restanti dall'altra istanza user3.
        // ATTENZIONE! Questo tipo di scorciatoia VA DICHIARATA SEMPRE PER ULTIMA.
        ..user3
    };
    // tuple struct
    // è possibile definire delle struct che somigliano a delle tuple. Come struttura sono
    // identiche a una tupla, con l'eccezione che hanno un nome.
    // Le struct tuple non hanno nomi dei campi ma occorre dichiararne il tipo, in questo modo:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 1, 2);
    // IMPORTANTE!
    // le variabili black e origin SONO DI TIPO DIFFERENTE PERCHÈ SONO ISTANZE DI DUE STRUCT
    // DIVERSE. pag.89
    //
    // # Unit-like structs - Le struct "vuote", senza campi.
    // è possibile dichiarare delle struct vuote, senza campi. Sono utili quando si implementano
    // dei trait senza dati da immagazzinare.

    struct AlwaysEqual;

    let subject = AlwaysEqual;
}
