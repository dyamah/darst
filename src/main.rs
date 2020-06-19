type BaseImpl = i32;
type CheckImpl = i32;
type Index = u32;


const ROOT: Index = 0;

trait TRIE {
    fn lookup(seq: &str, start: Index) -> Option<Index> ;
    fn common_prefix_search(seq: &str, func: fn(Index, Index) -> (), start: Index, end: Index);
}


mod double_array {
    use std::collections::HashMap;
    use crate::{BaseImpl, CheckImpl, Index, TRIE};
    use std::error::Error;
    use std::io;
    use std::fmt;
    use crate::double_array::DAError::UsedBaseValue;

    struct Node {
        base_impl: BaseImpl,
        check_impl: CheckImpl
    }

    struct DoubleArray {
        nodes: Vec<Node>,
        tails: Vec<Index>,
        codes: [Index]
    }

    enum DAError {
        UsedBaseValue,
        UsedCheckValue,
        DefinedNode,
    }

    const ROOT: Node = Node { base_impl: 0, check_impl: -1};

    impl Node {

        fn new() -> Node {
            Node { base_impl: -1, check_impl: -1}
        }

        fn base(&self) -> Option<Index> {
            match self.base_impl {
                b if b >= 0 => Some(b as Index),
                _ => None
            }
        }

        fn set_base(& mut self, value: Index) -> Result<&Node, DAError> {
            match self.base_impl {
                b if b <= 0 => {
                    self.base_impl = value as BaseImpl;
                    Ok(self)
                },
                _ => Err(DAError::UsedBaseValue)
            }
        }

        fn check(&self) ->  Option<Index> {
            match self.check_impl {
                c if c >= 0 => Some(c as Index),
                _ => None
            }
        }

        fn set_check(& mut self, value: Index) -> Result<&Node, DAError> {
            match self.check_impl {
                c if c <= 0 => {
                    self.check_impl = value as CheckImpl;
                    Ok(self)
                },
                _ => Err(DAError::UsedCheckValue)
            }
        }

        fn prev(self) -> Option<Index> {
            match self.base_impl {
                b if b < 0 => Some(-b as Index),
                _ => None
            }
        }

        fn set_prev(& mut self, value: Index) -> Result<&Node, DAError> {
            match self.base_impl {
                b if b < 0 => {
                    self.base_impl = -(value as BaseImpl);
                    Ok(self)
                },
                _ => Err(DAError::DefinedNode)
            }
        }

        fn next(self) -> Option<Index> {
            match self.check_impl {
                c if c < 0 => Some(-c as Index),
                _ => None
            }
        }

        fn set_next(& mut self, value: Index) -> Result<&Node, DAError> {
            match self.check_impl {
                c if c < 0 => {
                    self.base_impl = -(value as CheckImpl);
                    Ok(self)
                },
                _ => Err(DAError::DefinedNode)
            }
        }

        fn is_terminal(&self) -> bool { self.base_impl <= 0 && self.check_impl > 0 }

        fn tail(self) -> Option<Index> {
            if self.is_terminal() && self.base_impl < 0 {
                Some(-self.base_impl as Index)
            } else {
                None
            }
        }

        fn set_tail(& mut self, value: Index) -> Result<&Node, DAError> {
            if self.is_terminal() {
                self.base_impl = -(value as CheckImpl);
                Ok(self)
            } else {
                Err(DAError::DefinedNode)
            }
        }
    }
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for result in BufReader::new(File::open("sample.txt")?).lines() {
        let l = result?;

        println!("{}, len={}", l, l.len());
        for c in l.chars() {
            println!("{} ", c)
        }

    }
    Ok(())
}



