all: test

src/lib.out: src/lib.rs
	rustc --test src/lib.rs -o src/lib.out

test: src/lib.out
	./src/lib.out
