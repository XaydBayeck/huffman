mod tree;

use crate::tree::*;

fn main() {
    let tree1 = HuffmanTree::make_code_tree(
        HuffmanTree::make_leaf("A", 8),
        HuffmanTree::make_code_tree(
            HuffmanTree::make_code_tree(
                HuffmanTree::make_leaf("B", 3),
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf("C", 1),
                    HuffmanTree::make_leaf("D", 1),
                ),
            ),
            HuffmanTree::make_code_tree(
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf("E", 1),
                    HuffmanTree::make_leaf("F", 1),
                ),
                HuffmanTree::make_code_tree(
                    HuffmanTree::make_leaf("G", 1),
                    HuffmanTree::make_leaf("H", 1),
                ),
            ),
        ),
    );

    println!("{}", &tree1)
}
