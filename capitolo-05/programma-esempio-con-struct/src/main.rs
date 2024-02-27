// con questo programma (e i suoi vari refactoring) sarà possibile capire quando utilizzare le
// struct.
// Iniziamo con il creare un programma banale per calcolare l'area di un rettangolo (b x h).
fn main() {
    let width = 50;
    let height = 30;
    println!("L'area del rettangolo è: {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
