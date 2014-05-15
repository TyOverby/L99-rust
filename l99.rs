#![feature(macro_rules)]

#[deriving(Show, Eq)]
enum List<E> {
    Cons(E, Box<List<E>>),
    Nil
}

macro_rules! list(
    () => (Nil);
    ($x: expr) => (Cons($x, box Nil));
    ($x: expr, $($rest: expr),*) => (Cons($x, box list!($($rest),*)))
)

struct ListIter<E> {
    current: Option<List<E>>
}

impl <E> Iterator<E> for ListIter<E> {
    fn next(&mut self) -> Option<E> {
        let val = self.current.take();
        match val {
            Some(Cons(x, xs)) => {
                self.current = Some(*xs);
                Some(x)
            }
            Some(Nil) => None,
            None => None
        }
    }
}

impl <E> List<E> {
    fn iter(self) -> ListIter<E> {
        ListIter{ current: Some(self) }
    }
}

#[test]
fn test_iterator() {
    let v: Vec<uint> = vec![1,2,3,4];
    let l: List<uint>  = list![1,2,3,4];
    let v2 = l.iter().collect::<Vec<uint>>();
    assert!(v == v2);
}

impl <E> FromIterator<E> for List<E> {
    fn from_iter<T: Iterator<E>>(iterator: T) -> List<E> {
        let mut iter = iterator;
        let next = iter.next();
        match next {
            Some(x) => Cons(x, box FromIterator::from_iter(iter)),
            None => Nil
        }

    }
}

#[test]
fn from_iterator_test() {
    let v: Vec<uint> = vec![1,2,3,4];
    let l: List<uint>  = list![1,2,3,4];
    assert!(v.move_iter().collect::<List<uint>>() == l);
}

fn last<E>(list: List<E>) -> Option<E> {
    match list {
        Nil => None,
        Cons(x, box Nil) => Some(x),
        Cons(_, box xs) => last(xs),
    }
}

#[test]
fn test_last() {
    assert!(last(list!(1,2,3)) == Some(3));
    assert!(last::<()>(list!()) == None);
}

fn last_but_one<E>(list: List<E>) -> Option<E> {
    match list {
        Nil => None,
        Cons(_, box Nil) => None,
        Cons(x, box Cons(_, box Nil)) => Some(x),
        Cons(_, box xs) => last_but_one(xs)
    }
}
#[test]
fn test_last_but_one() {
    assert!(last_but_one(list!(1,2,3)) == Some(2));
    assert!(last_but_one(list!(3)) == None);
    assert!(last_but_one::<()>(list!()) == None);
}

fn kth<E>(list: List<E>, pos: uint) -> Option<E> {
    match (list, pos) {
        (Nil, _) => None,
        (Cons(x, _), 0) => Some(x),
        (Cons(_, box xs), _) => kth(xs, pos - 1)
    }
}
#[test]
fn test_kth() {
    assert!(kth(list!(1,2,3), 0) == Some(1));
    assert!(kth(list!(1,2,3), 5) == None);
    assert!(kth(list!(1,2,3), 2) == Some(3));
}

fn length<E>(list: List<E>) -> uint {
    match list {
        Nil => 0,
        Cons(_, box xs) => 1 + length(xs)
    }
}
#[test]
fn test_length() {
    assert!(length::<()>(list!()) == 0);
    assert!(length(list!(1,2,3)) == 3);
}

fn reverse<E>(list: List<E>) -> List<E> {
    let mut res = Nil;
    let mut from = list;
    loop {
        match from {
            Cons(x, box xs) => {
                res = Cons(x, box res);
                from = xs;
            }
            Nil => break
        }
    }
    res
}
#[test]
fn test_reverse() {
    assert!(reverse::<()>(list!()) == list!());
    assert!(reverse(list!(1,2,3)) == list!(3,2,1));
    assert!(reverse(list!(1,2)) == list!(2,1));
    assert!(reverse(list!(1)) == list!(1));
}


fn main() {}
