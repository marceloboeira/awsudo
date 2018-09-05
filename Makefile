CARGO_BIN = `which cargo`
TARGET_PATH = `pwd`/target/debug
BIN_NAME = awsudo
BIN_PATH = $(TARGET_PATH)/$(BIN_NAME)
INSTALLED_BIN_PATH = /usr/local/bin/$(BIN_NAME)

.PHONY: build
build:
	$(CARGO_BIN) build

.PHONY: run
run: build
	$(BIN_PATH)

.PHONY: install
install: build
	cp $(BIN_PATH) $(INSTALLED_BIN_PATH)

.PHONY: test
test:
	$(CARGO_BIN) test
