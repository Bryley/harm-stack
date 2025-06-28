
default:
    just --list

tailwind-watch:
    tailwindcss -i ./app/src/tailwind.css -o ./assets/tailwind.css --watch

cargo-watch:
    cargo watch -w ./app/src -w ./server/src -w ./assets -x run
