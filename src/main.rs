mod suffix_tree;
mod treatment;

use suffix_tree::SuffixTree;
fn main() {
    let mut tree = SuffixTree::new();
    tree.add_word(treatment::treat("saluton"));
    tree.add_word(treatment::treat("saluti"));
    tree.add_word(treatment::treat("sali"));
    println!("{}", tree);
}

#[cfg(test)]
mod test {
    use crate::suffix_tree::SuffixTree;
    use crate::treatment;
    #[test]
    fn add_remove() {
        let mut tree = SuffixTree::new();
        tree.add_word(treatment::treat("saluton"));
        tree.add_word(treatment::treat("saluti"));
        tree.add_word(treatment::treat("sali"));
        tree.delete_word(treatment::treat("saluti"));
        tree.delete_word(treatment::treat("neexista"));
        assert!(tree.contains_word(treatment::treat("sali")));
        assert!(!tree.contains_word(treatment::treat("saluti")));
        assert!(!tree.contains_word(treatment::treat("testi")));
    }
}
