# Set shell to bash
SHELL := /bin/bash

# Check if jq is installed
ifeq (,$(shell which jq))
$(error "jq is not installed. Please install it first")
endif

# Detect OS for sed compatibility
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    SED_IN_PLACE := sed -i ''
else
    SED_IN_PLACE := sed -i
endif

# Read ABI path from config.json using jq
ABI_PATH = $(shell jq -r '.contracts.brewboo_v2.abi_path' config.json)

# Get current ABI path from contracts.rs
CURRENT_ABI_PATH = $(shell sed -n 's/.*r"\([^"]*\)".*/\1/p' src/contracts.rs)

.PHONY: build
build:
	@echo "Current ABI path: $(CURRENT_ABI_PATH)"
	@echo "Config ABI path: $(ABI_PATH)"
	@if [ "$(CURRENT_ABI_PATH)" != "$(ABI_PATH)" ]; then \
		echo "Updating ABI path..." && \
		$(SED_IN_PLACE) 's|r"$(CURRENT_ABI_PATH)"|r"$(ABI_PATH)"|' src/contracts.rs && \
		cargo build; \
	else \
		echo "ABI paths are identical, proceeding with build..." && \
		cargo build; \
	fi
