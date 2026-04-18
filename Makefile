.PHONY: help db-up db-down db-logs db-shell db-reset build build-account run-account clean test fmt clippy

# Colors
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m

help: ## Show this help message
	@echo "$(BLUE)Cha-Trade - Available commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'

# ============================================
# Database Commands
# ============================================
db-up: ## Start database with docker-compose
	@echo "$(BLUE)Starting database...$(NC)"
	docker-compose up -d postgres

db-down: ## Stop database
	@echo "$(YELLOW)Stopping database...$(NC)"
	docker-compose down

db-logs: ## Show database logs
	docker-compose logs -f postgres

db-shell: ## Connect to database shell
	docker-compose exec postgres psql -U $(POSTGRES_USER:-cha_trade) -d $(POSTGRES_DB:-cha_trade)

db-reset: ## Reset database (WARNING: deletes all data)
	@echo "$(YELLOW)WARNING: This will delete all data!$(NC)"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [ $$REPLY = y ]; then \
		docker-compose down -v; \
		docker-compose up -d postgres; \
		echo "$(GREEN)Database reset complete$(NC)"; \
	fi

db-status: ## Show database status
	docker-compose ps postgres

# ============================================
# Build Commands
# ============================================
build: ## Build all services
	@echo "$(BLUE)Building all services...$(NC)"
	cargo build

build-account: ## Build account service
	@echo "$(BLUE)Building account service...$(NC)"
	cargo build -p account

build-release: ## Build all services in release mode
	@echo "$(BLUE)Building all services (release)...$(NC)"
	cargo build --release

# ============================================
# Run Commands
# ============================================
run-account: ## Run account service
	@echo "$(BLUE)Starting account service...$(NC)"
	cargo run -p account

run-account-debug: ## Run account service with debug logging
	RUST_LOG=debug cargo run -p account

# ============================================
# Development Commands
# ============================================
check: ## Run cargo check on all services
	cargo check

fmt: ## Format all code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features

test: ## Run all tests
	cargo test

test-account: ## Run account service tests
	cargo test -p account

# ============================================
# Clean Commands
# ============================================
clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean

clean-all: clean ## Clean everything including docker volumes
	@echo "$(YELLOW)Cleaning docker volumes...$(NC)"
	docker-compose down -v
