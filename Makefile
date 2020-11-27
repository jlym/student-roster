.PHONY: create-local-env
create-local-env:
	docker-compose \
		--file ./database/docker-compose.yml \
		up \
		--detach 

.PHONY: destroy-local-env
destroy-local-env:
	docker-compose \
		--file ./database/docker-compose.yml \
		down

.PHONY: db-session
db-session:
	psql "dbname=roster host=localhost user=postgres password=password port=5432"

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test
