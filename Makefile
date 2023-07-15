init: compose codegen topics 
	echo "Init done"

dev:
	cargo watch --watch ./src -x run

codegen:
	npx quicktype -s schema ./specs/message.jsonschema.json -o ./src/message.rs  --visibility public

topics:
	 docker-compose exec kafka kafka-topics --create --topic messages --bootstrap-server localhost:9092

compose: 
	docker-compose up -d

clean:
	docker-compose down