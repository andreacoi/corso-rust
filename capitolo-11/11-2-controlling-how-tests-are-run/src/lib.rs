pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]
    fn test_destinato_a_fallire() {
        let value = 5;
        assert_eq!(4, value);
    }

    /*
     * Eseguire test multipli
     * È possibile eseguire test multipli specificando una wildcard nel nome del test da eseguire.
     * Se, per esempio ho due test chiamati add_three e add_two e un terzo test chiamato sub_two,
     * posso far partire i test per add two e add_three digitando nel terminale cargo test add,
     * perché i nomi delle funzioni di cui eseguire il test iniziano per add.
     *
     * Per ignorare i test, è sufficiente specificare #[ignore] dopo il decorator #[test].
     * È possibile eseguire comunque i test ignorati passando l'argomento --ignored a cargo test.
     * */
}
