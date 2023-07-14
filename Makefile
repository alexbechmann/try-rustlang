codegen:
	npx quicktype -s schema ./specs/access-control.cloudevent.json -o ./src/access_control.rs  --visibility public

dev:
	npx nodemon --exec "cargo run" --watch src --watch Cargo.toml --ext rs