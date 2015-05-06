TEMPLATE=../../lempar.rs

src/parser.rs: src/parser.y ${TEMPLATE} lemon_rust
	./lemon_rust -T${TEMPLATE} $<

clean:
	rm -f lemon_rust src/parser.rs

lemon_rust: ../../lemon_rust.c
	gcc $< -o $@

