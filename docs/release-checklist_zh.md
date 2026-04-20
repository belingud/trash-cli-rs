# 发布检查清单

这份清单就是 `trash-cli-rs` 的发布流程。

## 必备输入

- 目标版本号
- 最终的 changelog / release notes 文案

## 检查清单

1. 检查 `CHANGELOG.md`，确认 `Unreleased` 真实反映当前变更。
2. 运行 `just release-check`。
3. 用锁定依赖图验证一次干净安装：
   `just install-smoke`
4. 确认 CLI 契约文档仍与真实行为一致：
   - `README.md`
   - `docs/rm-option-compatibility.md`
   - `docs/platform-differences.md`
5. 更新 `CHANGELOG.md`：
   - 将本次版本的内容从 `Unreleased` 移出
   - 新增一个带日期的版本小节
6. 生成发布提交和本地 tag：
   `just prepare-release <version>`
7. 在本地检查生成的 diff 和 tag。
8. 正式发布：
   `just publish <version>`
9. 确认 GitHub Actions 已为这个 tag 自动创建 GitHub Release。
10. 确认 release 已附带这些打包二进制：
    - Linux x64
    - Linux arm64
    - Windows x64
    - macOS arm64
    - macOS Intel

## 最低验收标准

- `cargo fmt --all --check` 通过
- `cargo test` 通过
- `cargo install --path . --locked` 能在临时 root 中成功安装
- 文档与实际发布行为一致

## 说明

- `just release-check` 是稳定的发布前检查入口。
- 现在只要 push 版本 tag，`.github/workflows/ci.yml` 里的 release job 就会在矩阵 CI 全部通过后自动创建 GitHub Release。
- 同一个 workflow 还会把各个支持目标的打包二进制自动上传为 GitHub Release 附件。
- 如果 CLI 契约发生变化，必须先更新文档再发布。
- 如果行为存在平台差异，必须先更新 `docs/platform-differences.md` 再发布。
