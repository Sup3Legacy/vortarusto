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
