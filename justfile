# Default variable
default := ''

# Run app
run external=default port=default port-number=default:
    cd frontend && astro build && cd .. && concurrently "RUSTRO_DEV=true cargo watch -x 'shuttle run {{external}} {{port}} {{port-number}}' -C backend" "cd frontend && npm run dev" -n Rust,Astro

# Build app
build external=default port=default port-number=default:
    cd frontend && astro build && cd .. && cd backend && cargo build --release && cd ..

# Preview app
preview external=default port=default port-number=default:
    cd frontend && astro build && cd .. && cargo watch -x 'shuttle run {{external}} {{port}} {{port-number}}' -C backend

#Deploy to shuttle
deploy:
    astro build --root=frontend && cargo shuttle deploy
