
# -------------------------------
# Stage 1: Build
# -------------------------------
FROM rust:1.93.1 as builder

# 设置工作目录
WORKDIR /usr/src/obs-presign

# 复制 Cargo.toml 并提前 build deps，加速构建缓存
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# 复制源代码
COPY . .

# 编译 release 二进制
RUN cargo build --release

# -------------------------------
# Stage 2: Runtime
# -------------------------------
FROM debian:bookworm-slim

# 安装必要的依赖（比如 libssl）
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 创建目录
WORKDIR /app

# 复制编译好的二进制
COPY --from=builder /usr/src/obs-presign/target/release/obs-presign .

RUN chmod +x /app/obs-presign

# 暴露端口
EXPOSE 3000

# 启动服务
CMD ["./obs-presign"]