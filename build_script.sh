#!/bin/bash

# Build script for Tic Tac Toe server

set -e

echo "🚀 Building Tic Tac Toe Server..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if Docker is installed (optional)
if command -v docker &> /dev/null; then
    echo "✅ Docker found"
    DOCKER_AVAILABLE=true
else
    echo "⚠️  Docker not found - manual build only"
    DOCKER_AVAILABLE=false
fi

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p static
mkdir -p config
mkdir -p docs
mkdir -p migration/src

# Build the project
echo "🔨 Building Rust project..."
cargo build --release

echo "✅ Build completed successfully!"

# Show next steps
echo ""
echo "🎯 Next steps:"
echo "  1. Run the server:     cargo run start"
echo "  2. Open browser:       http://localhost:5150"
echo "  3. Or use Docker:      docker-compose up --build"
echo ""
echo "🔧 Development commands:"
echo "  - Hot reload:          cargo watch -x run"
echo "  - Debug mode:          RUST_LOG=debug cargo run start"
echo "  - Run tests:           cargo test"
echo "  - Format code:         cargo fmt"
echo "  - Lint code:           cargo clippy"
echo ""
echo "📚 Documentation:"
echo "  - OpenAPI spec:        docs/openapi.yaml"
echo "  - AsyncAPI spec:       docs/asyncapi.yaml"
echo "  - README:              README.md"
echo ""
echo "🎮 Ready to play Tic Tac Toe!"
