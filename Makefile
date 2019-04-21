CARGO_BIN ?= `which cargo`
TARGET_PATH ?= `pwd`/target/release
BIN_VERSION ?= 0.1.1
BIN_NAME ?= awsudo
BIN_PATH ?= $(TARGET_PATH)/$(BIN_NAME)
NPM ?= `which npm`
MERMAID ?= `which mmdc`
DOCS_PATH ?= `pwd`/docs
RELEASE_FILE ?= $(BIN_NAME)-$(BIN_VERSION).x86_64-apple-darwin.tar.gz

.PHONY: build
build: format
	@$(CARGO_BIN) build

.PHONY: build_release
build_release: format
	@$(CARGO_BIN) build --release

.PHONY: format
format:
	@$(CARGO_BIN) fmt

.PHONY: run
run: build
	@$(BIN_PATH)

.PHONY: install
install: build_release
	$(CARGO_BIN) install --force

.PHONY: test
test: format
	@$(CARGO_BIN) test

.PHONY: setup_docs
setup_docs:
	@$(NPM) install -g mermaid.cli

.PHONY: docs
docs:
	@$(MERMAID) -i $(DOCS_PATH)/workflow.mmd -o $(DOCS_PATH)/workflow.png -t neutral

.PHONY: release
release: build_release
	@cd $(TARGET_PATH) && tar -zcvf ../../$(RELEASE_FILE) $(BIN_NAME)
	@shasum -a 256 $(RELEASE_FILE)
