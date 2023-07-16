INSTALL_PATH			:=$(HOME)/usr/bin/
CHORDATA_NAME			:=ms
CHORDATA_DEBUG_EXEC		:=target/debug/$(CHORDATA_NAME)
CHORDATA_RELEASE_EXEC		:=target/release/$(CHORDATA_NAME)
CHORDATA_EXEC			:=$(CHORDATA_RELEASE_EXEC)
CHORDATA_RUN			:=cargo run --bin $(CHORDATA_NAME) --
export K9_UPDATE_SNAPSHOTS	:=1
all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(CHORDATA_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(CHORDATA_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

release: check fix | $(CHORDATA_RELEASE_EXEC)
	install $(CHORDATA_RELEASE_EXEC) $(INSTALL_PATH)

debug: check fix | $(CHORDATA_DEBUG_EXEC)
	install $(CHORDATA_DEBUG_EXEC) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

fix run build test: check
	cargo $@

e2e: e2e-resolve e2e-chr e2e-ord

e2e-ord: debug
	$(CHORDATA_RUN) ord --dec O

e2e-resolve: debug
	$(CHORDATA_RUN) resolve 0x47 73 0x54

e2e-chr: debug
	$(CHORDATA_RUN) chr 0x54

# test $$(cargo run --bin ms -- ord --hex T) -eq 0x54
# test $$(cargo run --bin ms -- ord --dec T) -eq 84
# test $$(cargo run --bin ms -- ord --oct T) -eq 0o124
# test $$(cargo run --bin ms -- ord --bin T) -eq 0b1010100
# test $$(cargo run --bin ms -- chr --hex 0x54) -eq T
# test $$(cargo run --bin ms -- chr --dec 84) -eq T
# test $$(cargo run --bin ms -- chr --oct 0o124) -eq T
# test $$(cargo run --bin ms -- chr --bin 0b1010100) -eq T

.PHONY: all clean cls release debug fix fmt check build test examples run-$(CHORDATA_NAME)
