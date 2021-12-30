use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    Zero,
    One,
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::Zero => 0.fmt(f),
            _ => 1.fmt(f),
        }
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

    pub fn decode(&self, bits: &[Code]) -> Vec<String> {
        let mut current_branch = self;
        let mut result = vec![];

        for b in bits {
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

    fn choose_branch(&self, bit: &Code) -> &HuffmanTree {
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
}
