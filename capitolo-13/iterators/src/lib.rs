#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // il metodo sum() consuma l'iteratore, nel senso che ne "prende possesso", esegue le
    // operazioni e ritorna un valore, che io ho associato alla variabile totale.
    let total: u32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]

struct Scarpa {
    tipo: String,
    misura: u32,
}

fn scarpe_per_ogni_misura(scarpa: Vec<Scarpa>, misura_scarpa: u32) -> Vec<Scarpa> {
    scarpa
        .into_iter()
        .filter(|m| m.misura == misura_scarpa)
        .collect()
}
