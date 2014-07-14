#![desc = "A fizzbuzz kata in rust"]
#![license = "public domain"]

mod fizzlib;
#[cfg(test)]
mod test_fizzlib;

fn main() {
    for n in range(1, 15) {
        println!("{} : {}", n, fizzlib::fizzbuzz(&n))
    }
}
