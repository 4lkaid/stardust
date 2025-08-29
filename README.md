# stardust

[![CI](https://github.com/4lkaid/stardust/actions/workflows/ci.yaml/badge.svg)](https://github.com/4lkaid/stardust/actions/workflows/ci.yaml)

**「不碰业务 · 只盯流转」的轻量级资产状态追踪框架**

> “让每一次资产变更都有迹可循、有据可查”

## 配置文件示例

#### config.toml

```toml
[general]
listen = "0.0.0.0:8000"

[logger]
# Log levels: trace > debug > info > warn > error
# trace: Very detailed debugging information.
# debug: General debugging information.
# info: Normal operational information.
# warn: Potential issues.
# error: Serious problems.
level = "debug"
# writer options:
# file: Logs to "directory/file_name_prefix.year-month-day".
# stdout: Logs to console.
writer = "file"
directory = "./log"
file_name_prefix = "stardust.log"

[postgres]
url = "postgres://postgres:@127.0.0.1:5432/stardust"
max_connections = 10
min_connections = 1
acquire_timeout = 30  # seconds
idle_timeout = 600    # seconds
max_lifetime = 1800   # seconds

```

#### .env

```bash
DATABASE_URL="postgres://postgres:@127.0.0.1:5432/stardust"

```

## 许可证

本项目采用 MIT/Apache-2.0 双重授权模式（可任选其一遵循）：

- [MIT 许可证](LICENSE-MIT)
- [Apache 2.0 版许可证](LICENSE-APACHE)

完整法律声明请查阅对应许可证文件。
