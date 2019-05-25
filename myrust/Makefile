.PHONY=ALL
ALL: step0_repl.rs step1_read_print

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

