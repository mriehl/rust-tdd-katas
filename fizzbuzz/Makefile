all: test

src/test_fizzlib.out: src/test_fizzlib.rs
	rustc --test src/test_fizzlib.rs -o src/test_fizzlib.out

test: src/test_fizzlib.out
	./src/test_fizzlib.out

clean:
	rm -f src/test_fizzlib.out
