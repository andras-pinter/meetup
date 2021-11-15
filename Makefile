build:
	@docker-compose build

rebuild:
	@docker-compose build --no-cache

up: build
	@docker-compose up

attach:
	@docker-compose exec pam /bin/bash
