# trash-cli-rs

一个友好的命令行工具，用于将文件移动到回收站/垃圾箱，基于 `trash-rs` crate。
它首先设计用于安全删除，其次用于与别名（如 `alias rm="trash"`）良好配合。

提醒一下，新的 macOS（高于 macOS 14）已经内置了 trash 命令。

## 安装

```bash
cargo install trash-cli-rs
```

从源代码安装：

```bash
git clone https://github.com/belingud/trash-cli-rs.git
cd trash-cli-rs
cargo install --path .
```

## 使用

```bash
trash [rm-compat-options...] <file> [<file> ...]
trash --help
trash --version
```

## 项目范围

`trash-cli-rs` 不是 `rm` 的语义替换。
它忽略一组文档化的 `rm` 风格兼容性标志，以便常见的别名工作流程能够正常工作，
但它不实现 `rm` 语义，例如强制删除行为、交互式提示、
安全覆盖、白名单恢复或递归遍历逻辑。

公共兼容性契约位于：

- [docs/rm-option-compatibility_zh.md](docs/rm-option-compatibility_zh.md)
- [docs/platform-differences_zh.md](docs/platform-differences_zh.md)
- [docs/release-checklist_zh.md](docs/release-checklist_zh.md)

## 示例

```bash
trash file1.txt file2.txt
trash /path/to/file.txt
trash -rf build
trash --force --recursive build
trash -- -rf
trash ./-rf
```

## rm 别名兼容性

`trash-cli-rs` 设计用于与别名（如 `alias rm="trash"`）良好配合。
文档化的兼容性标志在操作数之前被忽略，因此剩余的路径仍会被移动到回收站。
未知的虚线参数被视为字面操作数，如果不存在这样的路径则会正常失败。
这种“按字面操作数处理”的行为是一个长期兼容性承诺，不是临时约定。

如果需要删除名称以 `-` 开头的文件，请使用显式分隔符或路径：

```bash
trash -- -rf
trash ./-rf
```

仅引用 `"-rf"` 是不够的，因为 shell 在将参数传递给程序之前会移除引号。

## 许可证

MIT
