#![feature(macro_rules)]

#[deriving(Show, Eq)]
enum List<E> {
    Cons(E, ~List<E>),
    Nil
}

macro_rules! list(
    () => (Nil);
    ($x: expr) => (Cons($x, ~Nil));
    ($x: expr, $($rest: expr),*) => (Cons($x, ~list!($($rest),*)))
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
        ListIter{current: Some(self)}
    }
}

fn last<E>(list: List<E>) -> Option<E> {
    match list {
        Nil => None,
        Cons(x, ~Nil) => Some(x),
        Cons(_, ~xs) => last(xs),
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
        Cons(_, ~Nil) => None,
        Cons(x, ~Cons(_, ~Nil)) => Some(x),
        Cons(_, ~xs) => last_but_one(xs)
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
        (Cons(_, ~xs), _) => kth(xs, pos - 1)
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
        Cons(_, ~xs) => 1 + length(xs)
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
            Cons(x, ~xs) => {
                res = Cons(x, ~res);
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
