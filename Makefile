.PHONY: up down build migrate migrate-dev log dev

up:       ## Start all services (builds if needed)
	docker compose up -d

down:     ## Stop all services
	docker compose down

build:    ## Build the app image
	docker compose build app

migrate:  ## Run database migrations (via Docker)
	docker compose run --rm db-migrate

migrate-dev:  ## Run migrations against local Postgres
	DATABASE_URL="postgres://postgres:postgres@localhost:5432/rok_api_dev" \
	  bash database/migrate.sh

log:      ## Tail app logs
	docker compose logs -f app

dev:      ## Run locally with cargo
	cargo run
