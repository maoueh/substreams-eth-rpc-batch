ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 19790028
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: map_block
map_block: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s $(START_BLOCK) -t $(STOP_BLOCK)
