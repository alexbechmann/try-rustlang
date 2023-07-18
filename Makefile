init: compose install codegen topics
	echo "Init done"

install: 
	cd tools && npm install
	cd libs/utils && cargo build
	cd apps/producer && cargo build
	cd apps/consumer && cargo build

codegen:
	cd tools && npm run convert
	jtd-codegen ./specs/event.jtd.json --rust-out ./libs/utils/src/event/
	jtd-codegen ./specs/access-control.jsonschema.jtd.json --rust-out ./libs/utils/src/access_control/ --root-name AccessControl
	npx -y quicktype@23.0.59 -s schema ./specs/message.jsonschema.json -o ./libs/utils/src/message.rs  --visibility public

topics:
	docker-compose exec kafka kafka-topics --create --topic messages --bootstrap-server localhost:9092 --if-not-exists

compose: 
	docker-compose up -d

clean:
	docker-compose down

test:
	cd apps/producer && cargo test
	cd apps/consumer && cargo test
	cd libs/utils && cargo test