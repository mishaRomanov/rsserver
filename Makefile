db_container_name = rsserver.db


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
	printf "\n\n\nrun\t -> run service\nrun_detached\t -> run in detached mode (with -d flag)\nkill\t -> stop all containers\nrerun\t -> self-explainatory. basically calls kill and then run\nlogs db\t -> look at db's logs in --follow mode"
