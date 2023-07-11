.PHONY: test build check release debug e2e e2e-encode e2e-ord e2e-chr

test build check:
	cargo $@

release:
	cargo build --$@
	install target/$@/ms ~/usr/bin

debug:
	cargo build
	install target/$@/ms ~/usr/bin

e2e: e2e-ord e2e-chr

e2e-ord: debug
	ms ord --hex T

e2e-chr: debug
	ms chr --hex 0x54


# test $$(cargo run --bin ms -- ord --hex T) -eq 0x54
# test $$(cargo run --bin ms -- ord --dec T) -eq 84
# test $$(cargo run --bin ms -- ord --oct T) -eq 0o124
# test $$(cargo run --bin ms -- ord --bin T) -eq 0b1010100
# test $$(cargo run --bin ms -- chr --hex 0x54) -eq T
# test $$(cargo run --bin ms -- chr --dec 84) -eq T
# test $$(cargo run --bin ms -- chr --oct 0o124) -eq T
# test $$(cargo run --bin ms -- chr --bin 0b1010100) -eq T
