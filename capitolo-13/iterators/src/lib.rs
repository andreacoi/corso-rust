#[derive(PartialEq, Debug)]

struct Scarpa {
    tipo: String,
    misura: u32,
}

fn scarpe_per_ogni_misura(scarpe: Vec<Scarpa>, misura_scarpa: u32) -> Vec<Scarpa> {
    scarpe
        .into_iter()
        .filter(|m| m.misura == misura_scarpa)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtra_scarpe_per_misura() {
        // avendo creato un vettore di struct, ho creato, quindi, un insieme da cui attingere per
        // fare le mie ricerche sui numeri disponibili. Quindi ho:
        // - delle ballerine numero 40;
        // - delle Sneakers numero 42;
        // - delle scarpe anti-infortunistica numero 42;
        // - degli stivali numero 39.
        // A questo punto devo verificare che la funzione che mi restituisce le scarpe per ogni
        // misura funzioni.
        // In breve devo verificare che indicando, ad esempio, il numero 39 la funzione
        // scarpe_per_ogni_misura mi restituisca un vettore con all'interno solo il paio di
        // stivali. Infatti, come 39 ho solo degli stivali.
        let scarpe = vec![
            Scarpa {
                tipo: String::from("Ballerine"),
                misura: 40,
            },
            Scarpa {
                tipo: String::from("Sneakers"),
                misura: 42,
            },
            Scarpa {
                tipo: String::from("Anti-Infortunistica"),
                misura: 42,
            },
            Scarpa {
                tipo: String::from("Stivali"),
                misura: 39,
            },
        ];
        //
        let scarpe_della_mia_misura = scarpe_per_ogni_misura(scarpe, 40);
        // eseguo la verifica del test
        assert_eq!(
            scarpe_della_mia_misura,
            vec![Scarpa {
                tipo: String::from("Ballerine"),
                misura: 40,
            }]
        )
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // il metodo sum() consuma l'iteratore, nel senso che ne "prende possesso", esegue le
        // operazioni e ritorna un valore, che io ho associato alla variabile totale.
        let total: u32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}
