use self::LinkedList::*;
use im::list::*;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T>{
    pub fn empty() -> Self {
		Tail
	}

	pub fn new(t: T) -> Self {
		Head(cons(t, List::new()))
	}

	pub fn push(self, t: T) -> Self {
		match self {
            Tail => Self::new(t),
            Head(list) => Head(cons(t, list)),
        }
    }

	pub fn push_back(&mut self, t: T) {
		match self {
			Tail => *self = Self::new(t),
			Head(list) => {
                let mut temp_list: im::List<T> = List::new();
                for i in list.iter() {
                    temp_list = cons(i, temp_list);
                }
                let mut new_list = cons(t, List::new());
                for i in temp_list.iter() {
                    new_list = cons(i, new_list);
                }
                *self = Head(new_list);
            }
		}
	}
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l, Head(cons(4, cons(3, List::new()))));
        l.push_back(2);
        assert_eq!(l, Head(cons(4, cons(3, cons(2, List::new())))));
    }
}

fn main() {}

// References:
// https://docs.rs/im/5.0.0/im/list/fn.cons.html
