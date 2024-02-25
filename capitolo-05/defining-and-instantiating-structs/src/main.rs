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
}
