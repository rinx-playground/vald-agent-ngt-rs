ORG = rinx
ALTORG = ghcr.io/rinx
REPO = vald-agent-ngt-rs

VALD_DIR = vald
VALD_REPO = vdaas/vald
VALD_BRANCH = feature/apis/v1-new-design
VALD_DEPTH = 1

DOCKER ?= docker
DOCKER_OPTS ?=

RUSTFLAGS ?= -Clink-arg=-fuse-ld=gold

.PHONY:
all: build

.PHONY: clean
clean:
	rm -rf \
	    target \
	    $(VALD_DIR)

.PHONY: build
build: \
	target/debug/vald-agent-ngt-rs

.PHONY: build/release
build/release: \
	target/release/vald-agent-ngt-rs


target/debug/vald-agent-ngt-rs: \
	proto \
	apis/proto/v1 \
	Cargo.toml \
	$(shell find ./src -type f -name '*.rs')
	cargo build

target/release/vald-agent-ngt-rs: \
	proto \
	apis/proto/v1 \
	Cargo.toml \
	$(shell find ./src -type f -name '*.rs')
	cargo build --release

proto: \
	proto/github.com/envoyproxy/protoc-gen-validate \
	proto/github.com/googleapis/googleapis

proto/github.com/envoyproxy/protoc-gen-validate:
	mkdir -p proto
	git clone --depth 1 \
	    https://github.com/envoyproxy/protoc-gen-validate \
	    proto/github.com/envoyproxy/protoc-gen-validate

proto/github.com/googleapis/googleapis:
	mkdir -p proto
	git clone --depth 1 \
	    https://github.com/googleapis/googleapis \
	    proto/github.com/googleapis/googleapis

apis/proto/v1: \
	proto \
	$(VALD_DIR)
	mkdir -p apis/proto
	cp -r $(VALD_DIR)/apis/proto/v1 $@

$(VALD_DIR):
	git clone \
	    --depth $(VALD_DEPTH) \
	    -b $(VALD_BRANCH) \
	    https://github.com/$(VALD_REPO) \
	    $(VALD_DIR)
