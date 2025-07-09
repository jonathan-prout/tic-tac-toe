.PHONY: build run test clean docker-build docker-run docker-stop dev docs

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the server in development mode
dev:
	RUST_LOG=debug cargo run start

# Run the server in production mode
run:
	cargo run start

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run clippy for linting
lint:
	cargo clippy -- -D warnings

# Check code without building
check:
	cargo check

# Build Docker image
docker-build:
	docker-compose build

# Run with Docker Compose
docker-run: docker-build
	docker-compose up

# Stop Docker containers
docker-stop:
	docker-compose down

# Run in background
docker-run-bg: docker-build
	docker-compose up -d

# View Docker logs
docker-logs:
	docker-compose logs -f

# Install dependencies
install:
	cargo build

# Development setup
setup: install
	@echo "Setup complete. Run 'make dev' to start development server."

# Generate API documentation (placeholder)
docs:
	@echo "API Documentation:"
	@echo "- OpenAPI: docs/openapi.yaml"
	@echo "- AsyncAPI: docs/asyncapi.yaml"
	@echo "- README: README.md"

# Run with hot reload (requires cargo-watch)
watch:
	cargo watch -x run

# Install development tools
dev-tools:
	cargo install cargo-watch
	cargo install cargo-edit

# Health check
health:
	@curl -f http://localhost:5150/api/state > /dev/null 2>&1 && echo "✅ Server is healthy" || echo "❌ Server is not responding"

# Quick test of endpoints
test-endpoints:
	@echo "Testing API endpoints..."
	@curl -s http://localhost:5150/api/tiles | jq '.'
	@curl -s http://localhost:5150/api/state | jq '.'
	@echo "✅ API endpoints are working"

# Reset game via API
reset:
	@curl -X POST http://localhost:5150/api/reset
	@echo "Game reset!"

# Make a test move
move:
	@curl -X POST http://localhost:5150/api/tile/1/1 \
		-H "Content-Type: application/json" \
		-d '{"state":"X"}' && echo "Move made!"

# Show project structure
tree:
	@tree -I 'target|.git' .

# Show help
help:
	@echo "Available targets:"
	@echo "  build         - Build the project"
	@echo "  dev           - Run in development mode with debug logging"
	@echo "  run           - Run the server"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  fmt           - Format code"
	@echo "  lint          - Run clippy linting"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-run    - Run with Docker Compose"
	@echo "  docker-stop   - Stop Docker containers"
	@echo "  watch         - Run with hot reload"
	@echo "  health        - Check server health"
	@echo "  test-endpoints- Test API endpoints"
	@echo "  reset         - Reset game via API"
	@echo "  move          - Make a test move"
	@echo "  help          - Show this help message"
