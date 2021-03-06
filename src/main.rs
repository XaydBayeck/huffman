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

    println!("{}", &tree1);

    let bits = Codes::from_str("011001011");
    let decode_information = tree1.decode(&bits);

    println!(
        "codes: {:?}, decode information: {:?}",
        bits.format(),
        &decode_information
    );

    let encodes = tree1.encode(&decode_information);

    println!("{}", encodes.format());

    let encode_pair = ["A", "NA", "BOOM", "SHA", "GET", "YIP", "JOB", "WAH"]
        .into_iter()
        .map(|s| String::from(s))
        .zip([2, 16, 1, 3, 2, 9, 2, 1].into_iter())
        .collect::<Vec<(String, i32)>>();

    println!("{:?}", &encode_pair);

    let tree2 = HuffmanTree::generate_huffman_tree(&encode_pair).unwrap();

    println!("{}", &tree2);

    let bits = Codes::from_str("0110101000111010100111101011001");
    let decode_message = tree2.decode(&bits);

    println!(
        "codes: {:?}, decode message: {:?}",
        bits.format(),
        &decode_message
    );

    println!("{}", tree2.encode(&decode_message).format())
}
