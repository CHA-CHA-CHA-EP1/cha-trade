.PHONY: help db-up db-down db-logs db-shell db-reset db-clean db-status \
        build build-account build-auth build-release \
        run-account run-account-debug run-auth run-auth-debug \
        check fmt clippy test test-account test-auth test-crypto \
        clean clean-all

# Colors
BLUE   := \033[0;34m
GREEN  := \033[0;32m
YELLOW := \033[0;33m
NC     := \033[0m

help: ## Show this help message
	@echo "$(BLUE)Cha-Trade - Available commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

# ============================================
# Database Commands
# ============================================
db-up: ## Start postgres
	@echo "$(BLUE)Starting database...$(NC)"
	docker-compose up -d postgres
	@echo "$(BLUE)Waiting for postgres to be ready...$(NC)"
	@until docker-compose exec postgres pg_isready -U $${POSTGRES_USER:-cha_trade} > /dev/null 2>&1; do sleep 1; done
	@echo "$(GREEN)Database ready$(NC)"

db-down: ## Stop all services
	@echo "$(YELLOW)Stopping services...$(NC)"
	docker-compose down

db-logs: ## Show postgres logs
	docker-compose logs -f postgres

db-shell: ## Connect to postgres shell
	docker-compose exec postgres psql -U $${POSTGRES_USER:-cha_trade} -d $${POSTGRES_DB:-cha_trade}

db-status: ## Show postgres status
	docker-compose ps postgres

db-clean: ## Remove postgres volume only (WARNING: deletes all data)
	@echo "$(YELLOW)WARNING: This will delete all postgres data!$(NC)"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [ "$$REPLY" = "y" ]; then \
		docker-compose stop postgres; \
		docker-compose rm -f postgres; \
		docker volume rm $$(docker volume ls -q | grep postgres_data) 2>/dev/null || true; \
		$(MAKE) db-up; \
		echo "$(GREEN)Database cleaned$(NC)"; \
	fi

db-reset: ## Reset all services including volumes (WARNING: deletes all data)
	@echo "$(YELLOW)WARNING: This will delete ALL data including kafka!$(NC)"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [ "$$REPLY" = "y" ]; then \
		docker-compose down -v; \
		$(MAKE) db-up; \
		echo "$(GREEN)Reset complete$(NC)"; \
	fi

# ============================================
# Build Commands
# ============================================
build: ## Build all services
	@echo "$(BLUE)Building all services...$(NC)"
	cargo build

build-account: ## Build account service
	@echo "$(BLUE)Building account service...$(NC)"
	cargo build -p account

build-auth: ## Build auth service
	@echo "$(BLUE)Building auth service...$(NC)"
	cargo build -p auth

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

run-auth: ## Run auth service
	@echo "$(BLUE)Starting auth service...$(NC)"
	cargo run -p auth

run-auth-debug: ## Run auth service with debug logging
	RUST_LOG=debug cargo run -p auth

# ============================================
# Development Commands
# ============================================
check: ## Run cargo check
	cargo check

fmt: ## Format all code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features

test: ## Run all tests
	cargo test

test-account: ## Run account service tests
	cargo test -p account

test-auth: ## Run auth service tests
	cargo test -p auth

test-crypto: ## Run crypto lib tests
	cargo test -p crypto

# ============================================
# Clean Commands
# ============================================
clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean

clean-all: clean ## Clean build artifacts and all docker volumes
	@echo "$(YELLOW)Cleaning docker volumes...$(NC)"
	docker-compose down -v
