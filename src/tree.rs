use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    Zero,
    One,
}

impl Code {
    pub fn new(bit: i32) -> Self {
        match bit {
            0 => Self::Zero,
            _ => Self::One,
        }
    }
}

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
    pub fn from_int(bits: &[i32]) -> Self {
        Self(bits.iter().map(|bit| Code::new(*bit)).collect())
    }

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

impl Display for HuffmanTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HuffmanTree::Leaf { symbol, weight } => write!(f, "({} {})", symbol, weight),
            HuffmanTree::Branch {
                symbols,
                weight,
                left,
                right,
            } => write!(f, "({:?} {})\n|-{}\n|-{}", symbols, weight, left, right),
        }
    }
}

impl HuffmanTree {
    pub fn make_leaf(symbol: &str, weight: i32) -> Self {
        HuffmanTree::Leaf {
            symbol: String::from(symbol),
            weight,
        }
    }

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
                    "This symbol: {} is not totolly in this tree's symbols:{:?}",
                    input_symbol, symbols
                )
            }
        }

        Codes(result)
    }

    pub fn encode(&self, messege: &[String]) -> Codes {
        if messege.is_empty() {
            Codes(vec![])
        } else {
            Codes(
                messege
                    .iter()
                    .map(|symbol| self.encode_symbol(symbol))
                    .fold(vec![], |mut v, mut cs| {
                        v.append(&mut cs.0);
                        v
                    }),
            )
        }
    }
}
