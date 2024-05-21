FILES = greeting

CUR_DIR=$(shell pwd)
OUT_FOLDER=bin
OUT_DIR=$(CUR_DIR)/$(OUT_FOLDER)

all: $(FILES)

.PHONY: $(FILES)
$(FILES):
	@RUSTFLAGS='-C link-arg=-zstack-size=262144 -C target-feature=+bulk-memory' cargo build --release --target=wasm32-unknown-unknown --features=$@ > /dev/null 2>&1
	@mkdir -p $(OUT_FOLDER)
	@cp ./target/wasm32-unknown-unknown/release/$@.wasm $(OUT_FOLDER)/$@.wasm
	@wasm2wat $(OUT_FOLDER)/$@.wasm > $(OUT_FOLDER)/$@.wat
	@$(MAKE) custom_file FILE_IN="$(OUT_DIR)/$@.wasm" FILE_OUT="$(OUT_DIR)/$@.rwasm" > /dev/null 2>&1
	@echo "wasm to rwasm created 🎉"
custom_file:
	@cp $(FILE_IN) $(FILE_OUT)
	
