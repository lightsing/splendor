# SDK

目前所支持的语言有：
| 语言                   | SDK                                                                        | 最低版本 |
|-----------------------|----------------------------------------------------------------------------|---------|
| Rust                  | [sdk/rust](https://github.com/lightsing/splendor/tree/master/sdk/rust)     |  1.76   |
| Python                | [sdk/py](https://github.com/lightsing/splendor/tree/master/sdk/py)         |  3.11   |
| Golang                | [sdk/go](https://github.com/lightsing/splendor/tree/master/sdk/go)         |  1.22   |

## 开发环境

每种语言推荐的开发环境是：
- Rust: 由 [Rustup](https://rustup.rs/) 安装的最新稳定版 Rust 与 [Cargo](https://doc.rust-lang.org/cargo/) 包管理器
- Python: [poetry](https://python-poetry.org/) 包管理器与其创建的虚拟环境
- Golang

除了语言本身的工具要求外，还需要安装以下工具：
- [Docker](https://www.docker.com/) 用于打包和测试运行游戏服务器
  - [Docker Compose](https://docs.docker.com/compose/) 用于本地测试运行游戏服务器