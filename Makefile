db_container_name = rsserver.db
help_file_name = help_txt
image_name = rsserver-server
# Should be parsed from env i think.
db_addr = postgres://backend_user:pass@rsserver.db:5432/logs?sslmode=disable

.PHONY: build rebuild run run_detached kill rerun logs env setup_db
help: 
	@echo "Makefile commands:"
	@echo "  build           - Build the docker images"
	@echo "  rebuild         - Rebuild the docker images from scratch"
	@echo "  run             - Run the docker containers"
	@echo "  run_detached    - Run the docker containers in detached mode"
	@echo "  kill            - Stop and remove the docker containers"
	@echo "  rerun           - Restart the docker containers"
	@echo "  logs db        - Follow the database container logs"
	@echo "  env setup_db   - Setup the .env file with database address"

build:
	docker-compose -f docker-compose.yml build
rebuild:
	docker image rm -f $(image_name) 
	docker-compose -f docker-compose.yml build
run:
	docker-compose -f docker-compose.yml up
run_detached:
	docker-compose -f docker-compose.yml up -d 
kill:
	docker-compose -f docker-compose.yml down 
rerun: kill run 
logs db: 
	docker logs --follow $(db_container_name)
env setup_db:
	echo 'DB_ADDR=$(db_addr)' > .env
