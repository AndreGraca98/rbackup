.DEFAULT_GOAL := help
.SILENT: 

install: ## Installing rust dependencies
	@echo "Installing dependencies..."
	@cargo install --path .
	
build: ## Build the project
	@echo "Building..."
	@cargo build --release

build-dev: ## Build the project in dev mode
	@echo "Building dev project..."
	@cargo build

LOGFILE ?= .mylogger.log
LOGLEVEL ?= info
run: build ## Build and run the project
	@echo "Running..."
	./target/release/rbackup a b --logfile $(LOGFILE) --loglevel $(LOGLEVEL)

run-dev: build-dev ## Build and run the project in dev mode
	@echo "Running dev project..."
	./target/debug/rbackup a b --logfile $(LOGFILE) --loglevel $(LOGLEVEL)

help: ## Display this help
	@echo "Usage: make [target]"
	awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / \
	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

	
	