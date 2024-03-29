# 比赛规则


1. 你仅可以提交一份 Actor 镜像且大小不得超过 4GB 并且上传至 Docker Hub。
2. 你的 Actor 镜像在启动时必须接受一个环境变量 `RPC_URL`，该变量指向游戏服务器的 WebSocket 地址。（这通常由SDK处理）
3. 你的 Actor 镜像在启动时必须接受一个环境变量 `CLIENT_SECRET`，该变量是由游戏服务器生成的一次性认证密钥。（这通常由SDK处理）
3. 你的 Actor 镜像在启动时可以接受一个环境变量 `STEP_TIMEOUT`，该变量指定了你的 Actor 在游戏中每一步的超时时间（秒）。
   该超时时间包括了游戏服务器与你的 Actor 之间的通信时间与应用操作的时间，因此你的 Actor 程序应当在此时间内给出响应。
4. 你的 Actor 程序不得访问除 `RPC_URL` 以外的任何网络资源。
5. 每次游戏中，你的 Actor 镜像将会被启动为一个容器，并且你的 Actor 程序将会被调用多次：
    1. 若你的 Actor 容器在游戏结束前意外退出或者断开Websocket连接，你会被判负，其他参赛者获胜。
    2. 若你的 Actor 给出了无效的行动或者游戏服务器不理解的响应，你会被判负，其他参赛者获胜。
    3. 若你的 Actor 在规定时间内未给出响应，你会被判负，其他参赛者获胜。
    4. 你可以对文件系统进行读写操作，但是你的 Actor 容器将在游戏结束后被销毁，因此你的文件系统操作将不会被保留。
6. 若某一轮中，所有参赛 Actor 都选择 Nop（即不做任何操作），则游戏结束，所有参赛者平局。

