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
            &Nil => String::from_str("<[]>"),
            &Cons(_, _) => String::from_str("<[").append(_repr_inner(list).as_slice())
        }
    }

    pub fn _repr_inner(list: &List) -> String{
        match list{
            &Nil => String::from_str("]>"),
            &Cons(number, box ref tail) if match tail{
                &Nil => true,
                _ => false,
            }=> number.to_str().append("]>"),
            &Cons(number, box ref tail) => number.to_str().append(", ").append(_repr_inner(tail).as_slice())
        }
    }

    pub fn equal(l1: &List, l2: &List) -> bool{
        match (l1, l2){
            (&Nil, &Nil) => true,
            (&Cons(n1, box ref tail1), &Cons(n2, box ref tail2)) if n1 == n2 => {
                equal(tail1, tail2)
            },
            _ => false,
        }
    }
}
