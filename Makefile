
create-local-env:
	docker-compose \
		--file ./database/docker-compose.yml \
		up \
		--detach 

destroy-local-env:
	docker-compose \
		--file ./database/docker-compose.yml \
		down

db-session:
	psql "dbname=roster host=localhost user=postgres password=password port=5432"
