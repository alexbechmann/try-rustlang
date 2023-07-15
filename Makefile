init: compose codegen topics
	echo "Init done"

install: 
	cd libs/utils && cargo build
	cd apps/try-rustlang && cargo build

codegen:
	cd libs/utils && npx quicktype -s schema ../../specs/message.jsonschema.json -o ./src/message.rs  --visibility public

topics:
	 docker-compose exec kafka kafka-topics --create --topic messages --bootstrap-server localhost:9092 --if-not-exists

compose: 
	docker-compose up -d

clean:
	docker-compose down