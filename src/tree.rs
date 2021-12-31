use std::fmt::Display;

/// 用于表示二进制编码
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    Zero,
    One,
}

// impl Code {
//     pub fn new(bit: i32) -> Self {
//         match bit {
//             0 => Self::Zero,
//             _ => Self::One,
//         }
//     }
// }

/// 二进制编码序列`Vec<Code>`的包装
#[derive(Debug, PartialEq)]
pub struct Codes(Vec<Code>);

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::Zero => 0.fmt(f),
            _ => 1.fmt(f),
        }
    }
}

impl Codes {
    // pub fn from_int(bits: &[i32]) -> Self {
    //     Self(bits.iter().map(|bit| Code::new(*bit)).collect())
    // }

    /// 从形如`"101001001"`类似的字符串生成`Codes`
    ///
    /// # Panics
    ///
    /// Panics if `bits` contains char which is not `'0'` or `'1'`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use self::Code::*;
    ///
    /// let codes = Codes::from_str("10011001");
    ///
    /// assert_eq!(codes， Codes(vec![One, Zero, Zero, One, One, Zero, Zero, One]))
    /// ```
    ///
    pub fn from_str(bits: &str) -> Self {
        Self(
            bits.chars()
                .map(|c| match c {
                    '0' => Code::Zero,
                    '1' => Code::One,
                    c => panic!("{} is not '0' or '1'", c),
                })
                .collect(),
        )
    }

    /// 将`Cdoes`以形如`"1001001010110"`类似的形式输出
    ///
    /// # Examples
    ///
    /// ```rust
    /// use self::Code::*;
    ///
    /// let codes = Codes::from_str("1001001010110");
    /// let codes_str = codes.format();
    ///
    /// assert_eq!(&codes_str, "1001001010110")
    /// ```
    pub fn format(&self) -> String {
        self.0
            .iter()
            .map(|c| match c {
                Code::Zero => '0',
                Code::One => '1',
            })
            .collect()
    }
}

/// 表示Huffman编码的二叉树结构
///
/// # Examples
///
/// 你可以通过``一张权重表来生成`HuffmanTree`
///
/// 1. 构建权重表
///
/// ```rust
/// let encode_pair =
/// [("A", 2),
/// ("NA", 16),
/// ("BOOM", 1),
/// ("SHA", 3),
/// ("GET", 2),
/// ("YIP", 9),
/// ("JOB", 2),
/// ("WAH", 1)]
///
/// 2. 生成`HuffmanTree`
///
/// ```rust
/// let tree = HuffmanTree::generate_huffman_tree(&encode_pair).unwrap();
///
/// println!("{}", &tree);
///
/// ```
///
/// - Output
/// ```
/// ([NA, YIP, WAH, BOOM, JOB, SHA, GET, A] 36)
/// |-L:(NA 16)
/// |-R:([YIP, WAH, BOOM, JOB, SHA, GET, A] 20)
/// |    |-L:(YIP 9)
/// |    |-R:([WAH, BOOM, JOB, SHA, GET, A] 11)
/// |    |    |-L:([WAH, BOOM, JOB] 4)
/// |    |    |    |-L:([WAH, BOOM] 2)
/// |    |    |    |    |-L:(WAH 1)
/// |    |    |    |    |-R:(BOOM 1)
/// |    |    |    |-R:(JOB 2)
/// |    |    |-R:([SHA, GET, A] 7)
/// |    |    |    |-L:(SHA 3)
/// |    |    |    |-R:([GET, A] 4)
/// |    |    |    |    |-L:(GET 2)
/// |    |    |    |    |-R:(A 2)
///
/// ```
///
#[derive(Debug, Clone)]
pub enum HuffmanTree {
    Leaf {
        symbol: String,
        weight: i32,
    },
    Branch {
        symbols: Vec<String>,
        weight: i32,
        left: Box<HuffmanTree>,
        right: Box<HuffmanTree>,
    },
}

/// 专门用于格式化输出Huffman编码树的包装类
struct HuffmanTreeShow<'a>(i32, &'a HuffmanTree);

impl Display for HuffmanTreeShow<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = (0..self.0)
            .map(|_| "|\t")
            .fold(String::new(), |acc, x| acc + x);

        match &self.1 {
            HuffmanTree::Leaf { symbol, weight } => write!(f, "({} {})", symbol, weight),
            HuffmanTree::Branch {
                symbols,
                weight,
                left,
                right,
            } => write!(
                f,
                "([{}] {})\n{}|-L:{}\n{}|-R:{}",
                symbols.join(", "),
                weight,
                &depth,
                &HuffmanTreeShow(self.0 + 1, left),
                &depth,
                &HuffmanTreeShow(self.0 + 1, right)
            ),
        }
    }
}

impl Display for HuffmanTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        HuffmanTreeShow(0, self).fmt(f)
    }
}

impl HuffmanTree {
    /// 用一个`symbol: &str`和`weight: i32`来生成一个叶节点
    pub fn make_leaf(symbol: &str, weight: i32) -> Self {
        HuffmanTree::Leaf {
            symbol: String::from(symbol),
            weight,
        }
    }

    /// 用俩个`HuffmanTree`分别作为左子树、右子树自动归并生成一棵新树
    pub fn make_code_tree(left: HuffmanTree, right: HuffmanTree) -> Self {
        let (mut left_symbols, left_weight) = left.get_weight_and_symbols();
        let (mut right_symbols, right_weight) = right.get_weight_and_symbols();

        left_symbols.append(&mut right_symbols);

        Self::Branch {
            left: Box::new(left),
            right: Box::new(right),
            weight: left_weight + right_weight,
            symbols: left_symbols,
        }
    }

    /// 获取当前`HuffmanTree`节点的符号表和权重（当当前节点为叶节点时，会将该叶节点的符号装入一个`Vec`中）
    fn get_weight_and_symbols(&self) -> (Vec<String>, i32) {
        match self {
            HuffmanTree::Leaf { symbol, weight } => (vec![symbol.clone()], weight.clone()),
            HuffmanTree::Branch {
                symbols,
                weight,
                left: _,
                right: _,
            } => (symbols.clone(), weight.clone()),
        }
    }

    /// 对`0`、`1`序列进行解码
    ///
    /// # Examples
    ///
    /// ```rust
    /// let tree = ...;
    ///
    /// let codes = Codes::from_str("1001000110");
    ///
    /// let decode_message = tree.decode(&codes);
    ///
    /// ```
    pub fn decode(&self, bits: &Codes) -> Vec<String> {
        let mut current_branch = self;
        let mut result = vec![];

        for b in &bits.0 {
            let next_branch = current_branch.choose_branch(b);
            if let HuffmanTree::Leaf { symbol, weight: _ } = next_branch {
                result.push(symbol.to_string());
                current_branch = self
            } else {
                current_branch = next_branch
            }
        }

        result
    }

    /// 当`bit`为`Zero`时选择左子树;当`bit`为`One`时选择右子树
    fn choose_branch(&self, bit: &Code) -> &Self {
        if let Self::Branch {
            symbols: _,
            weight: _,
            left,
            right,
        } = self
        {
            match bit {
                Code::Zero => left,
                Code::One => right,
            }
        } else {
            self
        }
    }

    /// 获取当前节点的符号表
    fn get_symbols(&self) -> Vec<String> {
        match self {
            HuffmanTree::Leaf { symbol, weight: _ } => vec![symbol.to_string()],
            HuffmanTree::Branch {
                symbols,
                weight: _,
                left: _,
                right: _,
            } => symbols.to_vec(),
        }
    }

    /// 得到`input_symbol`在当前`HuffmanTree`中的编码
    ///
    /// # Panics
    ///
    /// 当`input_symbol`不在树中或者树的编码存在错误的情况下发生 **Panics**
    fn encode_symbol(&self, input_symbol: &String) -> Codes {
        let mut result = vec![];

        let mut current_branch = self;

        while let Self::Branch {
            symbols,
            weight: _,
            left,
            right,
        } = current_branch
        {
            if symbols.contains(input_symbol) {
                if left.get_symbols().contains(input_symbol) {
                    result.push(Code::Zero);
                    current_branch = left;
                } else if right.get_symbols().contains(input_symbol) {
                    result.push(Code::One);
                    current_branch = right;
                } else {
                    panic!("ERROR!: This branch's symbols({:?}) are not completely contained by left({:?}) and right({:?})", symbols, left.get_symbols(), right.get_symbols())
                }
            } else {
                panic!(
                    "This symbol: {} is not totally in this tree's symbols:{:?}",
                    input_symbol, symbols
                )
            }
        }

        Codes(result)
    }

    /// 将输入的数据用当前的树进行编码
    pub fn encode(&self, message: &[String]) -> Codes {
        if message.is_empty() {
            Codes(vec![])
        } else {
            Codes(
                message
                    .iter()
                    .map(|symbol| self.encode_symbol(symbol))
                    .fold(vec![], |mut v, mut cs| {
                        v.append(&mut cs.0);
                        v
                    }),
            )
        }
    }

    /// 获取当前节点的权重
    fn get_weight(&self) -> i32 {
        match self {
            HuffmanTree::Leaf { symbol: _, weight } => *weight,
            HuffmanTree::Branch {
                symbols: _,
                weight,
                left: _,
                right: _,
            } => *weight,
        }
    }

    /// 通过符号权重表生成一棵`HuffmanTree`
    ///
    /// # Return Option
    ///
    /// - **`None`** : 当输入的表为空时
    /// - **`Some(HuffmanTree)`** : 其他情况
    ///
    pub fn generate_huffman_tree(pairs: &[(String, i32)]) -> Option<Self> {
        if pairs.is_empty() {
            None
        } else {
            let mut trees = pairs
                .iter()
                .map(|(symbol, weight)| Self::make_leaf(symbol, *weight))
                .collect::<Vec<Self>>();

            let sort_leafs =
                |leafs: &mut Vec<Self>| leafs.sort_by(|a, b| b.get_weight().cmp(&a.get_weight()));

            sort_leafs(&mut trees);

            while let Some((left, right)) = trees.pop().and_then(|left| {
                trees
                    .pop()
                    .and_then(|right| Some((left.clone(), right.clone())))
                    .or_else(|| {
                        trees.push(left.clone());
                        None
                    })
            }) {
                trees.push(Self::make_code_tree(left, right));
                sort_leafs(&mut trees);
            }

            trees.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_from_str_test() {
        use Code::*;

        let codes = Codes::from_str("100110110101");

        assert_eq!(
            codes,
            Codes(vec![
                One, Zero, Zero, One, One, Zero, One, One, Zero, One, Zero, One
            ])
        )
    }

    #[test]
    fn codes_format_test() {
        let codes = Codes::from_str("100110110101");

        assert_eq!(&codes.format(), "100110110101");
    }

    #[test]
    fn huffman_tree_decode_encode_test() {
        let tree = HuffmanTree::generate_huffman_tree(
            &[
                ("A", 8),
                ("B", 3),
                ("C", 1),
                ("D", 1),
                ("E", 1),
                ("F", 1),
                ("G", 1),
                ("H", 1),
            ]
            .map(|(s, i)| (String::from(s), i)),
        )
        .unwrap();

        let unencode_message = vec!["A", "E", "D"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let encode_bits = tree.encode(&unencode_message);

        let decode_message = tree.decode(&encode_bits);

        assert_eq!(decode_message, unencode_message)
    }
}
