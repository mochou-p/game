<!-- mochou-p/game/README.md -->

# game
experimental game infrastructure/ecosystem

## workspace
crate         | what it is
--------------|---------------------------------------------
game/core     | player and gameplay related data
game/protocol | network messages, serialization, transmition
game/client   | native game
game/server   | TCP and UDP multiplayer game server
game/bots     | stress testing for game/server
web/backend   | HTTP web server
web/frontend  | HTML, CSS
database/core | SQLite
utils         | logging macros, reusable generic types

## usage

### game
```sh
cargo run --bin game_client --release
```
(prepend with `WAYLAND_DISPLAY=""` to use xwayland instead of wayland on linux)

### server
```sh
cargo run --bin game_server --release
```

### website
```sh
cargo run --bin web_backend --release
```

