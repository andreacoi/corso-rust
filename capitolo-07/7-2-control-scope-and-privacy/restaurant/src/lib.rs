// Creiamo un nuovo modulo
// Ho anche creato dei moduli annidati:
// - accoglienza è un modulo che gestisce l'accoglienza dei clienti (fa parte sempre del frontend
// di un ristorante)
// - servizio è un modulo che gestisce le dinamiche relative al servizio vero e proprio in sala.
// Ovviamente ogni modulo è suddiviso in funzioni che vanno ad agire in maniera più atomica nel
// contesto del modulo.
mod frontend_ristorante {
    mod accoglienza {
        fn popola_lista_attesa() {}
        fn fai_accomodare_clienti() {}
    }
    mod servizio {
        fn prendi_comanda() {}
        fn servi_portate() {}
        fn riscuoti_pagamento() {}
    }
}
