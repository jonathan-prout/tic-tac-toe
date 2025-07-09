# Tic Tac Toe - REST + WebSocket Demo

A demonstration of unified REST and WebSocket APIs using Rust and Loco.rs framework. This project shows how to build APIs where REST endpoints handle direct actions while WebSockets provide real-time updates - both operating on the same underlying game state.

## Architecture

- **REST API**: Direct game actions (GET tiles, POST moves, POST reset)
- **WebSocket API**: Real-time updates with topic-based messaging
- **Unified State**: Both APIs operate on the same in-memory game state
- **Topic-based Updates**: WebSocket messages use `game.tile.{x}.{y}` and `game.state` topics

## Features

- 3x3 Tic Tac Toe game
- Player selection (X or O)
- Real-time updates via WebSocket
- Game reset functionality
- Win condition detection
- OpenAPI and AsyncAPI specifications
- Docker containerization

## Technology Stack

- **Backend**: Rust with Loco.rs framework
- **WebSocket**: Native Rust WebSocket with topic-based messaging
- **Frontend**: Vanilla JavaScript with Autobahn.js
- **API Documentation**: OpenAPI 3.0 and AsyncAPI 3.0
- **Containerization**: Docker and Docker Compose

## API Documentation

### REST API

The REST API provides direct game actions:

- `GET /api/tiles` - Get current 3x3 grid state
- `GET /api/state` - Get current win condition
- `POST /api/tile/{x}/{y}` - Make a move (returns 200 or 409)
- `POST /api/reset` - Reset the game

### WebSocket API

The WebSocket API provides real-time updates using topic-based messaging:

- **Connection**: `ws://localhost:5150/ws`
- **Topics**:
  - `game.tile.{x}.{y}` - Tile state changes
  - `game.state` - Win condition changes

#### Message Format

```json
{
  "topic": "game.tile.1.2",
  "payload": {
    "state": "X"
  }
}
```

## Quick Start

### Using Docker (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd tictactoe-server

# Build and run with Docker Compose
docker-compose up --build

# Access the game
open http://localhost:5150
```

### Manual Build

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone <repository-url>
cd tictactoe-server
cargo build --release

# Run the server
cargo run start

# Access the game
open http://localhost:5150
```

## Development

### Project Structure

```
├── src/
│   ├── app.rs              # Application configuration
│   ├── game.rs             # Game logic and state
│   ├── websocket.rs        # WebSocket manager
│   ├── controllers/
│   │   ├── game.rs         # REST API endpoints
│   │   └── websocket.rs    # WebSocket handler
│   ├── main.rs
│   └── lib.rs
├── static/
│   └── index.html          # Frontend application
├── config/
│   ├── development.yaml    # Development configuration
│   └── production.yaml     # Production configuration
├── docs/
│   ├── openapi.yaml        # OpenAPI specification
│   └── asyncapi.yaml       # AsyncAPI specification
├── migration/              # Database migrations (placeholder)
├── Dockerfile
├── docker-compose.yml
└── README.md
```

### Running Tests

```bash
cargo test
```

### Development Mode

```bash
# Run with hot reload
cargo watch -x run

# Run with debug logging
RUST_LOG=debug cargo run start
```

## API Specifications

### OpenAPI (REST)

The OpenAPI specification is available at:
- File: `docs/openapi.yaml`
- Runtime: `http://localhost:5150/docs` (when implemented)

### AsyncAPI (WebSocket)

The AsyncAPI specification is available at:
- File: `docs/asyncapi.yaml`
- Shows topic-based messaging structure
- Documents real-time event flows

## Game Flow

1. **Page Load**: 
   - Frontend calls `GET /api/tiles` and `GET /api/state`
   - Establishes WebSocket connection
   - Receives initial game state via WebSocket

2. **Making Moves**:
   - User clicks tile → `POST /api/tile/{x}/{y}`
   - If successful → WebSocket publishes `game.tile.{x}.{y}` event
   - If win condition changes → WebSocket publishes `game.state` event

3. **Game Reset**:
   - User clicks reset → `POST /api/reset`
   - WebSocket publishes events for all tiles and state

## Configuration

### Environment Variables

- `RUST_LOG`: Logging level (debug, info, warn, error)
- `LOCO_ENV`: Environment (development, production)

### Configuration Files

- `config/development.yaml`: Development settings
- `config/production.yaml`: Production settings

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Common Issues

1. **Port 5150 already in use**:
   ```bash
   # Change port in config files or use different port
   docker-compose up --build -p 5151:5150
   ```

2. **WebSocket connection failed**:
   - Check that server is running
   - Verify WebSocket URL in browser console
   - Check firewall settings

3. **Build errors**:
   - Ensure Rust version is up to date
   - Clean and rebuild: `cargo clean && cargo build`

### Debug Mode

Enable debug logging to see detailed WebSocket and game state information:

```bash
RUST_LOG=debug cargo run start
```

This will show:
- WebSocket connection/disconnection events
- Game state changes
- API request/response details
- Error messages and stack traces
