# Clean — cleanup commands for caches and build artifacts

# 暴力清理所有 cargo 缓存（慢，下次全量重建）
clean:
    cargo clean

# 智能清理 7 天未访问的构建缓存
clean-sweep:
    just _require cargo-sweep "cargo install cargo-sweep"
    cargo sweep --time 7

# 清理不在当前 Cargo.toml 中的依赖缓存
clean-sweep-deps:
    just _require cargo-sweep "cargo install cargo-sweep"
    cargo sweep --installed

# 仅清理覆盖率产物
clean-coverage:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    cargo llvm-cov clean

# 查看 sccache 缓存状态（当前不可用）
sccache-stats:
    @echo "sccache currently disabled — see .env for details"

# 清理 sccache 缓存（当前不可用）
sccache-clean:
    @echo "sccache currently disabled — see .env for details"
