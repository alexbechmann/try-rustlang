codegen:
	npx quicktype -s schema ./specs/message.jsonschema.json -o ./src/message.rs  --visibility public

dev:
	npx nodemon --exec "cargo run" --watch src --watch Cargo.toml --ext rs

topics:
	 docker-compose exec kafka kafka-topics --create --topic messages --bootstrap-server localhost:9092