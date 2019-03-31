CARGO_BIN ?= `which cargo`
TARGET_PATH ?= `pwd`/target/debug
BIN_NAME ?= awsudo
BIN_PATH ?= $(TARGET_PATH)/$(BIN_NAME)
INSTALLED_BIN_PATH = /usr/local/bin/$(BIN_NAME)
NPM ?= `which npm`
MERMAID ?= `which mmdc`
DOCS_PATH ?= `pwd`/docs

.PHONY: build
build: format
	@$(CARGO_BIN) build

.PHONY: format
format:
	@$(CARGO_BIN) fmt

.PHONY: run
run: build
	@$(BIN_PATH)

.PHONY: install
install: build
	@cp $(BIN_PATH) $(INSTALLED_BIN_PATH)
	@echo "You can try running 'awsudo' now"

.PHONY: test
test:
	@$(CARGO_BIN) test

.PHONY: setup_docs
setup_docs:
	@$(NPM) install -g mermaid.cli

.PHONY: docs
docs:
	@$(MERMAID) -i $(DOCS_PATH)/workflow.mmd -o $(DOCS_PATH)/workflow.png -t neutral
