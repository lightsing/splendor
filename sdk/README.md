# Spendor Actor SDK

## Available Languages

| Language              | SDK                    | Status | Maintainer  |
|-----------------------|------------------------|--------|-------------|
| Rust                  | [sdk/rust](./rust)     | 🟢 ✅    | @lightsing  |
| Python                | [sdk/py](./py)         | 🟢 ✅    | @lightsing  |
| Golang                | [sdk/go](./go)         | 🟢 ✅    | @lightsing  |
| C++                   | [sdk/cxx](./cxx)       | 🟡 📅    | @Ray-Eldath |
| Java/Kotlin           | [sdk/jvm](./jvm)       | 🟡 📅    | @sorz       |
| JavaScript/TypeScript | [sdk/js](./js)         | 🟡 📅    | @Almsev     |
| C#                    | [sdk/csharp](./csharp) | 🔴 📅    | N/A         |


- 🟢: Maintained by core maintainers
- 🟡: Maintained by community contributers
- 🔴: No mantainers
- ✅: Ready to Use
- 🔧: Under developing
- 📅: Plan to support

## Common ENV Variables

Parameters are inject to the environment variables.

|      Name     |                    Explain                    |    Default Value    |
|:-------------:|:---------------------------------------------:|:-------------------:|
|    RPC_URL    |         The game websocket server url.        |   ws://server:8080  |
| CLIENT_SECRET | The one time auth secret generated by server. | /app/secrets/secret |
