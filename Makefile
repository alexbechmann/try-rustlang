init: compose install codegen topics
	echo "Init done"

install: 
	cd libs/utils && cargo build
	cd apps/producer && cargo build
	cd apps/consumer && cargo build

build: install

codegen:
	quicktype -s schema ./specs/message.jsonschema.json -o ./libs/utils/src/message.rs  --visibility public

topics:
	docker-compose exec kafka kafka-topics --create --topic messages --bootstrap-server localhost:9092 --if-not-exists

compose: 
	docker-compose up -d

clean:
	docker-compose down

test:
	cd libs/utils && cargo test
	cd apps/producer && cargo test
	cd apps/consumer && cargo test

fix: 
	cd libs/utils && cargo fix --allow-dirty
	cd apps/producer && cargo fix --allow-dirty
	cd apps/consumer && cargo fix --allow-dirty
