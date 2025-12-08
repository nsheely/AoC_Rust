.PHONY: help check clean
.PHONY: test test-2023 test-2024 test-2025
.PHONY: bench bench-2023 bench-2024 bench-2025
.PHONY: download download-all download-2023-all download-2024-all download-2025-all
.PHONY: fmt clippy fix fix-warnings all

# Default target
.DEFAULT_GOAL := help

# Years we're working with
YEARS := 2023 2024 2025

##@ General

help: ## Display this help message
	@echo "Advent of Code - Rust Solutions"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make <target>\n"} /^[a-zA-Z_0-9%-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 } /^##@/ { printf "\n%s\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

check: ## Check all code compiles
	@cd 2023 && cargo check
	@cd 2024 && cargo check
	@cd 2025 && cargo check

clean: ## Clean build artifacts
	@cd 2023 && cargo clean
	@cd 2024 && cargo clean
	@cd 2025 && cargo clean

##@ Testing

test: test-2023 test-2024 test-2025 ## Run all solutions for all years

test-2023: ## Run all 2023 solutions
	@echo "=== Testing 2023 ==="
	@cd 2023 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2023/day$$day_num.txt" ]; then \
			cargo aoc -d $$day_num 2>&1 | grep -E "^Day|Part" || true; \
		fi; \
	done

test-2024: ## Run all 2024 solutions
	@echo "=== Testing 2024 ==="
	@cd 2024 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2024/day$$day_num.txt" ]; then \
			cargo aoc -d $$day_num 2>&1 | grep -E "^Day|Part" || true; \
		fi; \
	done

test-2025: ## Run all 2025 solutions
	@echo "=== Testing 2025 ==="
	@cd 2025 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2025/day$$day_num.txt" ]; then \
			cargo aoc -d $$day_num 2>&1 | grep -E "^Day|Part" || true; \
		fi; \
	done

test-2023-day%: ## Run specific 2023 day (e.g., make test-2023-day1)
	@cd 2023 && cargo aoc -d $(subst test-2023-day,,$@)

test-2024-day%: ## Run specific 2024 day (e.g., make test-2024-day1)
	@cd 2024 && cargo aoc -d $(subst test-2024-day,,$@)

test-2025-day%: ## Run specific 2025 day (e.g., make test-2025-day1)
	@cd 2025 && cargo aoc -d $(subst test-2025-day,,$@)

##@ Benchmarking

bench: bench-2023 bench-2024 bench-2025 ## Benchmark all days for all years

bench-2023: ## Benchmark all 2023 solutions
	@echo "=== Benchmarking 2023 ==="
	@cd 2023 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2023/day$$day_num.txt" ]; then \
			echo "Day $$day_num:"; \
			cargo aoc bench -d $$day_num -g 2>&1 | awk '/^Generator.*Part/ {p=$$0; getline; if (/time:/) print p $$0} /^Day.*time:/ {print}'; \
			echo ""; \
		fi; \
	done

bench-2024: ## Benchmark all 2024 solutions
	@echo "=== Benchmarking 2024 ==="
	@cd 2024 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2024/day$$day_num.txt" ]; then \
			echo "Day $$day_num:"; \
			cargo aoc bench -d $$day_num -g 2>&1 | awk '/^Generator.*Part/ {p=$$0; getline; if (/time:/) print p $$0} /^Day.*time:/ {print}'; \
			echo ""; \
		fi; \
	done

bench-2025: ## Benchmark all 2025 solutions
	@echo "=== Benchmarking 2025 ==="
	@cd 2025 && \
	for day_file in $$(ls src/day*.rs 2>/dev/null | sort -V); do \
		day_num=$$(echo $$day_file | sed 's/src\/day//;s/.rs//'); \
		if [ -f "input/2025/day$$day_num.txt" ]; then \
			echo "Day $$day_num:"; \
			cargo aoc bench -d $$day_num -g 2>&1 | awk '/^Generator.*Part/ {p=$$0; getline; if (/time:/) print p $$0} /^Day.*time:/ {print}'; \
			echo ""; \
		fi; \
	done

bench-2023-day%: ## Benchmark specific 2023 day (e.g., make bench-2023-day1)
	@cd 2023 && cargo aoc bench -d $(subst bench-2023-day,,$@) -g

bench-2024-day%: ## Benchmark specific 2024 day (e.g., make bench-2024-day1)
	@cd 2024 && cargo aoc bench -d $(subst bench-2024-day,,$@) -g

bench-2025-day%: ## Benchmark specific 2025 day (e.g., make bench-2025-day1)
	@cd 2025 && cargo aoc bench -d $(subst bench-2025-day,,$@) -g

##@ Input Management

download: ## Download input (requires YEAR and DAY, e.g., make download YEAR=2024 DAY=1)
	@if [ -z "$(YEAR)" ] || [ -z "$(DAY)" ]; then \
		echo "Usage: make download YEAR=2024 DAY=1"; \
		exit 1; \
	fi
	@cd $(YEAR) && cargo aoc input -d $(DAY)

download-all: download-2023-all download-2024-all download-2025-all ## Download all inputs for all years

download-2023-all: ## Download all 2023 inputs (days 1-25)
	@for day in {1..25}; do \
		cd 2023 && cargo aoc input -d $$day || true; \
		sleep 1; \
	done

download-2024-all: ## Download all 2024 inputs (days 1-25)
	@for day in {1..25}; do \
		cd 2024 && cargo aoc input -d $$day || true; \
		sleep 1; \
	done

download-2025-all: ## Download all 2025 inputs (days 1-25)
	@for day in {1..25}; do \
		cd 2025 && cargo aoc input -d $$day || true; \
		sleep 1; \
	done

##@ Quick Shortcuts (current year: 2025)

day%: ## Quick test for 2025 (e.g., make day1)
	@cd 2025 && cargo aoc -d $(subst day,,$@)

benchday%: ## Quick benchmark for 2025 (e.g., make benchday1)
	@cd 2025 && cargo aoc bench -d $(subst benchday,,$@) -g

##@ Development

fmt: ## Format all code
	@cd 2023 && cargo fmt
	@cd 2024 && cargo fmt
	@cd 2025 && cargo fmt

clippy: ## Run clippy lints
	@cd 2023 && cargo clippy
	@cd 2024 && cargo clippy
	@cd 2025 && cargo clippy

fix: ## Apply automatic fixes
	@cd 2023 && cargo fix --allow-dirty --allow-staged && cargo fmt
	@cd 2024 && cargo fix --allow-dirty --allow-staged && cargo fmt
	@cd 2025 && cargo fix --allow-dirty --allow-staged && cargo fmt

fix-warnings: ## Fix all warnings
	@cd 2023 && cargo fix --allow-dirty --allow-staged && cargo clippy --fix --allow-dirty --allow-staged
	@cd 2024 && cargo fix --allow-dirty --allow-staged && cargo clippy --fix --allow-dirty --allow-staged
	@cd 2025 && cargo fix --allow-dirty --allow-staged && cargo clippy --fix --allow-dirty --allow-staged

##@ Common Workflows

all: check test bench ## Check, test, and benchmark everything
	@echo "=== All checks passed! ==="
