init: compose install codegen topics
	echo "Init done"

install: 
	cd libs/utils && cargo build
	cd apps/producer && cargo build
	cd apps/consumer && cargo build

codegen:
	cd libs/utils && npx -y quicktype@23.0.59 -s schema ../../specs/message.jsonschema.json -o ./src/message.rs  --visibility public
	jtd-codegen ./specs/event.jtd.json --rust-out ./libs/utils/src/event.rs 

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