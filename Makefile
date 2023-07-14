codegen:
	npx quicktype -s schema ./specs/message.jsonschema.json -o ./src/message.rs  --visibility public

dev:
	npx nodemon --exec "cargo run" --watch src --watch Cargo.toml --ext rs