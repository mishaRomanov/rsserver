db_container_name = rsserver.db
help_file_name = help_txt

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
