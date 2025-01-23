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

# Read ABI paths from config.json using jq
ABI_PATH_V2 = $(shell jq -r '.contracts.brewboo_v2.abi_path' config.json)
ABI_PATH_V3 = $(shell jq -r '.contracts.brewboo_v3.abi_path' config.json)

# Get current ABI paths from contracts.rs
CURRENT_ABI_PATH_V2 = $(shell sed -n 's/.*r"\([^"]*v2.*\)".*/\1/p' src/contracts.rs)
CURRENT_ABI_PATH_V3 = $(shell sed -n 's/.*r"\([^"]*v3.*\)".*/\1/p' src/contracts.rs)

.PHONY: build
build:
	@echo "Current ABI_V2 path: $(CURRENT_ABI_PATH_V2)"
	@echo "Current ABI_V3 path: $(CURRENT_ABI_PATH_V3)"
	@echo "Config ABI_V2 path: $(ABI_PATH_V2)"
	@echo "Config ABI_V3 path: $(ABI_PATH_V3)"
	@if [ "$(CURRENT_ABI_PATH_V2)" != "$(ABI_PATH_V2)" ]; then \
		echo "Updating ABI_V2 path..." && \
		$(SED_IN_PLACE) 's|r"$(CURRENT_ABI_PATH_V2)"|r"$(ABI_PATH_V2)"|' src/contracts.rs && \
	else \
		echo "ABI_V2 paths are identical, proceeding with build..."\
	fi
	@if [ "$(CURRENT_ABI_PATH_V3)" != "$(ABI_PATH_V3)" ]; then \
    	echo "Updating ABI_V3 path..." && \
    	$(SED_IN_PLACE) 's|r"$(CURRENT_ABI_PATH_V3)"|r"$(ABI_PATH_V3)"|' src/contracts.rs && \
	else \
		echo "ABI_V3 paths are identical, proceeding with build..."\
	fi
	@cargo build
