#![desc = "A linked list kata in rust"]
#![license = "public domain"]
#![feature(box_syntax)]

#[cfg(test)]
mod linkedlist_test;
mod linkedlist{

    pub enum List{
        Cons(int, Box<List>),
        Nil
    }

    pub fn visual_compare(l1: &List, l2: &List) -> String{
        let equality = match equal(l1, l2){
            true  => "",
            false => "not ",
        };
        format!("{} and {} are {}equal", repr(l1), repr(l2), equality)
    }

    pub fn repr(list: &List) -> String{
        match list{
            &List::Nil => String::from_str("<[]>"),
            &List::Cons(_, _) => {
                let mut nested_repr = String::from_str("<[");
                nested_repr.push_str(_repr_inner(list).as_slice());
                nested_repr
            }
        }
    }

    fn _repr_inner(list: &List) -> String{
        match list{
            &List::Nil => String::from_str("]>"),
            &List::Cons(number, box ref tail) if match tail{
                &List::Nil => true,
                _ => false,
            }=> {
                let mut tail_repr = number.to_string();
                tail_repr.push_str("]>");
                tail_repr
            }
            &List::Cons(number, box ref tail) => {
                let mut loose_tail = number.to_string();
                loose_tail.push_str(", ");
                loose_tail.push_str(_repr_inner(tail).as_slice());
                loose_tail
            }
        }
    }

    pub fn equal(l1: &List, l2: &List) -> bool{
        match (l1, l2){
            (&List::Nil, &List::Nil) => true,
            (&List::Cons(n1, box ref tail1), &List::Cons(n2, box ref tail2)) if n1 == n2 => {
                equal(tail1, tail2)
            },
            _ => false,
        }
    }
}
