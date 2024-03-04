# Splendor Game Server

This package is for run with other player actor via websocket.

## Build

```bash
docker build -t splendor-server:latest -f Dockerfile ..
```

## Enviroment Variables

|       Name        | Explain                                                               |          Possible Values           |
|:-----------------:|:----------------------------------------------------------------------|:----------------------------------:|
|     N_PLAYERS     | The number many actors are used in this game                          |              2, 3, 4               |
|   SECRETS_PATH    | The generated websocket secret path, used for actor client to connect |            a valid path            |
|    SERVER_ADDR    | The listen address of the websocket server                            | a socket address, eg. 0.0.0.0:8080 |
|    RANDOM_SEED    | The random seed to deterministically reproduce the game.              |           an u64 integer           |
|      GAME_ID      | The game uuid                                                         |              an uuid               |
| SUPERVISOR_SOCKET | The supervisor grpc socket path                                       |            a valid path            |
|    STEP_TIMEOUT   | The timeout for each player's step in seconds                         |           a positive u64           |

## Secrets

The secrets are wrote to `$SECRETS_PATH/player$idx/secret`, make sure map the dir to actor container.
