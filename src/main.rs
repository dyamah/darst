

type BaseImpl = i32;
type CheckImpl = i32;
type Index = u32;

const ROOT: Index = 0;

trait TRIE {
    type T;

    fn lookup(seq: &[N], start: Index) -> Option<Index> ;
    fn comon_prefix_search(seq: &[N], func: fn(Index, Index) -> Unit, start: Index, end: Index);
}


mod double_array {
    use crate::{BaseImpl, CheckImpl, Index, TRIE};
    use std::option::NoneError;

    struct Node {
        base: BaseImpl,
        check: CheckImpl
    }

    struct DoubleArray<T> {
        nodes: Vec<Node>,
        tails: Vec<Index>,
        codes: [T]
    }

    impl Node {

        fn base(self) -> Option<Index> {
            None
        }

        fn set_base(self, value: Index) -> ! {
        }

        fn check(self) ->  Option<Index> {
            None
        }

        fn set_check(self, value: Index) -> ! {
        }

        fn prev(self) -> Option<Index> {
            None
        }

        fn set_prev(self, value: Index) -> ! {
        }

        fn next(self) -> Option<Index> {
            None
        }

        fn set_next(self, value: Index) -> ! {
        }

        fn is_traminal(self) -> bool {
            false
        }

        fn tail(self) -> Option<Index> {
            None
        }

        fn set_tail(self, value: Index) -> ! {
        }
    }

    impl DoubleArray<T> {
        fn create_code_table(data: u32) -> Vec<T> {
        }

        fn new(data: &Vec<String>) -> DoubleArray<Char> {
        }
    }

}
