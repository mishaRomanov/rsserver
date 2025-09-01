db_container_name = rsserver.db
help_file_name = help_txt
image_name = rsserver-server

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
help:
	head $(help_file_name)
