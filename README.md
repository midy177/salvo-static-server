# Salvo Static Server

一个基于 Rust 和 Salvo 框架的高性能静态文件服务器，支持 HTTPS、HTTP/3、自动证书管理和响应压缩。

## ✨ 特性

- 🚀 **高性能**：基于 Rust 异步运行时
- 🔒 **HTTPS 支持**：自动 ACME 证书管理和续签
- 🌐 **HTTP/3 支持**：QUIC 协议支持
- 📦 **多静态目录**：支持多个静态资源目录
- 🗜️ **响应压缩**：默认启用 Gzip 压缩
- 📝 **请求日志**：详细的访问日志记录
- 🔄 **强制 HTTPS**：HTTP 自动重定向到 HTTPS
- 🎯 **灵活配置**：丰富的命令行参数
- 🐳 **Docker 支持**：优化的 Docker 镜像

## 🚀 快速开始

### 本地运行

```bash
# 克隆仓库
git clone https://github.com/yourusername/salvo-static-server.git
cd salvo-static-server

# 构建项目
cargo build --release

# 运行服务器
./target/release/salvo-static-server
```

### 使用 Makefile

```bash
# 安装开发依赖
make install-dev

# 创建示例文件
make setup

# 开发模式（热重载）
make dev

# 构建发布版本
make build
```

### Docker 运行

```bash
# 构建 Docker 镜像
make docker-build

# 运行容器
make docker-run

# Docker Compose
docker-compose up -d
```

## 📖 使用方法

### 基本用法

```bash
# 默认配置（HTTP 8080 端口）
salvo-static-server

# 自定义端口
salvo-static-server --http-addr 0.0.0.0:3000

# 启用压缩（默认开启）
salvo-static-server

# 禁用压缩
salvo-static-server --no-compression
```

### HTTPS 配置

```bash
# 启用 HTTPS + ACME 证书
salvo-static-server \
  --https-enable \
  --acme-domain yourdomain.com \
  --https-addr 0.0.0.0:8443

# 强制 HTTPS（HTTP 重定向到 HTTPS）
salvo-static-server \
  --https-enable \
  --acme-domain yourdomain.com \
  --force-https

# 启用 HTTP/3
salvo-static-server \
  --https-enable \
  --acme-domain yourdomain.com \
  --http3-enable
```

### 高级配置

```bash
# 多个静态目录
salvo-static-server \
  --static-dir /var/www/html \
  --static-dir /var/www/assets \
  --static-dir /var/www/media

# 自定义默认文件和 404 页面
salvo-static-server \
  --try-file index.html \
  --default-page custom-404.html \
  --static-dir ./public

# 完整生产配置
salvo-static-server \
  --http-addr 0.0.0.0:80 \
  --https-addr 0.0.0.0:443 \
  --https-enable \
  --acme-domain yourdomain.com \
  --force-https \
  --http3-enable \
  --log-level info
```

## ⚙️ 配置选项

| 参数 | 默认值 | 描述 |
|------|--------|------|
| `--http-addr` | `0.0.0.0:8080` | HTTP 监听地址 |
| `--https-addr` | `0.0.0.0:8443` | HTTPS 监听地址 |
| `--https-enable` | `false` | 启用 HTTPS |
| `--force-https` | `false` | 强制 HTTPS 重定向 |
| `--http3-enable` | `false` | 启用 HTTP/3 (QUIC) |
| `--acme-domain` | - | ACME 证书域名 |
| `--static-dir` | `["static"]` | 静态资源目录（可多个） |
| `--try-file` | `index.html` | 默认文件名 |
| `--default-page` | `404.html` | 404 页面 |
| `--no-compression` | `false` | 禁用响应压缩 |
| `--log-level` | `info` | 日志级别 |

## 🐳 Docker 部署

### 基本部署

```bash
# 构建镜像
docker build -t salvo-static-server .

# 运行容器
docker run -d \
  --name salvo-static \
  -p 8080:8080 \
  -v $(pwd)/static:/app/static \
  salvo-static-server
```

### 生产环境

```bash
# HTTPS 生产部署
docker run -d \
  --name salvo-static-prod \
  -p 80:8080 \
  -p 443:8443 \
  -v $(pwd)/static:/app/static \
  -v $(pwd)/certs:/app/temp \
  --restart unless-stopped \
  salvo-static-server \
  --https-enable \
  --acme-domain yourdomain.com \
  --force-https
```

### Docker Compose

```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "8080:8080"
      - "8443:8443"
    volumes:
      - ./static:/app/static
      - ./temp:/app/temp
    environment:
      - RUST_LOG=info
    command: 
      - "--https-enable"
      - "--acme-domain=yourdomain.com"
      - "--force-https"
    restart: unless-stopped
```

## 📊 性能

### 基准测试

```bash
# 使用 wrk 进行压测
wrk -t12 -c400 -d30s http://localhost:8080/

# 使用 ab 进行测试
ab -n 10000 -c 100 http://localhost:8080/
```

### 性能优化

- ✅ **响应压缩**：默认启用 Gzip 压缩
- ✅ **HTTP/2 和 HTTP/3**：多路复用支持
- ✅ **零拷贝**：高效的文件传输
- ✅ **异步 I/O**：Rust 异步运行时

## 🛠️ 开发

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化代码
cargo fmt
```

### 开发工具

```bash
# 安装开发依赖
make install-dev

# 热重载开发
make dev

# 调试模式
make dev-debug
```

## 📋 TODO

- [ ] 支持 Brotli 和 Zstd 压缩
- [ ] 添加缓存控制头配置
- [ ] 支持虚拟主机
- [ ] 添加 WebDAV 支持
- [ ] 集成 Prometheus 指标
- [ ] 支持 S3 存储后端

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Salvo](https://salvo.rs/) - 优秀的 Rust Web 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Let's Encrypt](https://letsencrypt.org/) - 免费 SSL 证书

## 📞 支持

- 📧 邮箱：support@example.com
- 🐛 问题反馈：[GitHub Issues](https://github.com/yourusername/salvo-static-server/issues)
- 💬 讨论：[GitHub Discussions](https://github.com/yourusername/salvo-static-server/discussions)

---

⭐ 如果这个项目对你有帮助，请给它一个星标！