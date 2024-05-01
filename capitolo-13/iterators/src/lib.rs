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
        let scarpe = vec![
            Scarpa {
                tipo: String::from("Ballerine"),
                misura: 40,
            }, // TODO: INSERIRE ALTRI MODELLI DI SCARPE CON ULTERIORI MISURE E PROVARE LA FUNZIONE.
        ];
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
