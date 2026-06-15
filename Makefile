ifneq (,$(wildcard ./.env))
    include .env
    export
endif

SERVER=apps/server
CLIENT=apps/client
app ?= "server"
service ?= $(app)

.DEFAULT_GOAL := run
.PHONY: run detach clean fmt lint migration machete clean-db db setup down

setup:
	rm -rf $(CLIENT)/node_modules
	cd $(CLIENT) && pnpm install --frozen-lockfile --ignore-scripts
	docker compose build
	docker compose up client -d --force-recreate --renew-anon-volumes

run:
	docker compose up

down:
	docker compose down

detach:
	docker compose up -d

clean:
	docker compose down -v

clean-db:
	docker exec orcid-acad-mgr-postgres psql -U user -d database -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	cd $(SERVER) && sqlx migrate run --source ./config/migrations --database-url $(LOCAL_POSTGRES_DATABASE_URL)

db:
	docker exec -it orcid-acad-mgr-postgres /bin/bash -c "psql -U user -d database"

fmt:
	cargo fmt --all
	cd $(CLIENT) && pnpm run format

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd $(CLIENT) && pnpm run lint

migration:
	cd $(SERVER) && sqlx migrate add --source ./config/migrations "$(name)"

machete:
	cargo machete
