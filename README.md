# Splendor

Splendor is a game of chip-collecting and card development.

This repository implements the game logic in Rust and also provides various language sdk for developers to build their own AI to play the game.

## Project Structure

- `splendor-core`: The game core types implemented in Rust.
- `splendor-engine`: The game engine implemented in Rust.
- `splendor-server`:
  A one time game server implemented in Rust, which serves WebSocket.
  It's designed to serve the game for a single game session and run in Docker.
- `splendor-supervisor`:
  A supervisor implemented in Rust, which creates and game containers
  also the resource limit for each game container.
- `splendor-proto`:
  Internal gRPC protocol for the game server and the supervisor.
- [`sdk`](./sdk): Various language SDKs for developers to build their own AI to play the game.

### SDKs

Each SDK provides an example AI which takes random actions.

> ***Note***: The Rust example AI is implemented in `actor::naive_actors` in the `splendor-core` crate.

## Run the Game Server Locally

See the [docker-compose.yml](./docker-compose.yml).

It starts the game server without the supervisor.

