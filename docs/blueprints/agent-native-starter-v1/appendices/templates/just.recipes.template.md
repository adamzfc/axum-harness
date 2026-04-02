# Justfile 配方模板

```just
set shell := ["bash", "-lc"]

default:
  @just --list

setup:
  proto install
  bun install

doctor:
  moon run repo:doctor

dev:
  moon run repo:dev-fullstack

typegen:
  moon run repo:typegen

verify:
  moon run repo:verify

test:
  moon run repo:test-unit
  moon run repo:test-integration
  moon run repo:test-e2e

evals:
  moon run repo:evals-run

release:
  moon run repo:release-dry-run
```

## 设计要点

- Just 是统一入口，不是复杂编排中心
- 常用命令可读、可记、可自动化
