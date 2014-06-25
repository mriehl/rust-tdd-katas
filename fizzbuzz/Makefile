all: clean test

src/fizzbuzz.out: src/fizzbuzz.rs
	rustc --test src/fizzbuzz.rs -o src/fizzbuzz.out

test: src/fizzbuzz.out
	./src/fizzbuzz.out

clean:
	rm -f src/fizzbuzz.out
