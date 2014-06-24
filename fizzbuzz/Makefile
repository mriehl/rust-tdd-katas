all: test

src/test_fizzbuzz.out: src/test_fizzbuzz.rs
	rustc --test src/test_fizzbuzz.rs -o src/test_fizzbuzz.out

test: src/test_fizzbuzz.out
	./src/test_fizzbuzz.out

clean:
	rm -f src/test_fizzbuzz.out
