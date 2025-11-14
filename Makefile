# Variables
APP_NAME := bin
BUILD_DIR := target
CONFIG_DIR := $(HOME)/.config/$(APP_NAME)
LOG_FILE := $(HOME)/.local/share/$(APP_NAME)/$(APP_NAME).log

# Rust toolchain
CARGO := cargo

# Colors for output
YELLOW := \033[1;33m
GREEN := \033[1;32m
RESET := \033[0m

# Default target
.DEFAULT_GOAL := help

# some paths
LOCAL_LOGS = ~/.local/share/minirs/
CONFIG_PATH = ~/.config/minirs/

# Targets
.PHONY: help
help:
	@echo -e "$(GREEN)Available commands:$(RESET)"
	@echo -e "$(YELLOW)  make build$(RESET)      Build the project in release mode."
	@echo -e "$(YELLOW)  make run$(RESET)        Build and run the project."
	@echo -e "$(YELLOW)  make clean$(RESET)      Remove build artifacts."
	@echo -e "$(YELLOW)  make install$(RESET)    Install the application to /usr/local/bin."
	@echo -e "$(YELLOW)  make uninstall$(RESET)  Uninstall the application."
	@echo -e "$(YELLOW)  make config$(RESET)     Generate a default configuration file."
	@echo -e "$(YELLOW)  make logs$(RESET)       Tail the log file."
	@echo -e "$(YELLOW)  make fmt$(RESET)        Format the Rust code."
	@echo -e "$(YELLOW)  make lint$(RESET)       Run the linter and treat warnings as errors."
	@echo -e "$(YELLOW)  make test$(RESET)       Run tests for the project."
	@echo -e "$(YELLOW)  make help$(RESET)       Show this help message."

# Targets
.PHONY: build
build:
	@echo -e "$(YELLOW)Building the project...$(RESET)"
	$(CARGO) build --release
	@echo -e "$(GREEN)Build completed!$(RESET)"

.PHONY: run
run: build
	@echo -e "$(YELLOW)Running the application...$(RESET)"
	$(CARGO) run

.PHONY: clean
clean:
	@echo -e "$(YELLOW)Cleaning build files...$(RESET)"
	$(CARGO) clean
	@echo -e "$(GREEN)Clean completed!$(RESET)"

.PHONY: install
install: build
	@echo -e "$(YELLOW)Installing the application...$(RESET)"
	install -d $(CONFIG_DIR)
	install -m 755 $(BUILD_DIR)/release/$(APP_NAME) /usr/local/bin/$(APP_NAME)
	@echo -e "$(YELLOW)Copying default configuration...$(RESET)"
	install -d $(CONFIG_PATH)
	$(APP_NAME) --generate-config
	@echo -e "$(GREEN)Installation completed!$(RESET)"

.PHONY: uninstall
uninstall:
	@echo -e "$(YELLOW)Uninstalling the application...$(RESET)"
	rm -f /usr/local/bin/$(APP_NAME)
	@echo -e "$(GREEN)Uninstallation completed!$(RESET)"

.PHONY: config
config:
	@echo -e "$(YELLOW)Generating default configuration...$(RESET)"
	mkdir -p $(CONFIG_DIR)
	$(APP_NAME) --generate-config
	@echo -e "$(GREEN)Configuration generated at $(CONFIG_DIR).$(RESET)"

.PHONY: logs
logs:
	@echo -e "$(YELLOW)Displaying logs...$(RESET)"
	tail -f $(LOG_FILE)

.PHONY: fmt
fmt:
	@echo -e "$(YELLOW)Formatting the code...$(RESET)"
	$(CARGO) fmt
	@echo -e "$(GREEN)Code formatted!$(RESET)"

.PHONY: lint
lint:
	@echo -e "$(YELLOW)Running linter...$(RESET)"
	$(CARGO) clippy -- -D warnings
	@echo -e "$(GREEN)Linter completed!$(RESET)"

.PHONY: test
test:
	@echo -e "$(YELLOW)Running tests...$(RESET)"
	$(CARGO) test
	@echo -e "$(GREEN)Tests completed!$(RESET)"

.PHONY: rm
rm:
	@echo -e "$(YELLOW)Remove MiniRs directories...$(RESET)"
	rm -rf $(LOCAL_LOGS) $(CONFIG_PATH)
	@echo -e "$(GREEN)Directories removed!$(RESET)"
