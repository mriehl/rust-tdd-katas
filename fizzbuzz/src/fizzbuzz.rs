mod fizzlib;
mod test_fizzlib;

fn main() {
    for n in range(1, 15) {
        println!("{} : {}", n, fizzlib::fizzbuzz(&n))
    }
}
