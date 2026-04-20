# 平台差异

本文档记录 `trash-cli-rs` 中真正会影响用户感知的平台差异。

## 当前验证状态

| 平台    | 上游后端存在 | 本项目 CI 已验证 | 说明                                                             |
| ------- | ------------ | ---------------- | ---------------------------------------------------------------- |
| macOS   | 是           | 是               | 当前在 CI 和本地开发中都有实际验证。                             |
| Linux   | 是           | 是               | 当前在 CI 中有实际验证。                                         |
| Windows | 是           | 已配置 CI        | Windows 已通过 `windows-2025` 加入 GitHub Actions 矩阵；等第一轮远程运行变绿后再算真正验证完成。 |

## macOS

`trash-cli-rs` 当前依赖 `trash` crate 的 macOS 后端。
在 `trash 5.2.5` 中，这个后端提供两种删除方式：

- `Finder`
- `NSFileManager`

默认方式是 `Finder`，而 `trash-cli-rs` 当前没有覆盖这个默认值。

实际影响：

- 文件通过 Finder 语义进入废纸篓。
- 操作可能表现出 Finder 本身的 UI 副作用。
- 上游文档说明 `NSFileManager` 更快、更安静，但在某些系统上可能失去 Finder 的 “Put Back” 行为。

使用的源码依据：

- `trash-5.2.5/src/macos/mod.rs`

## Linux

在 Linux 上，上游后端遵循 Freedesktop Trash Specification 1.0。

实际影响：

- 回收站位置与挂载点相关。
- 行为依赖于期望的 trash 目录是否存在、是否可访问。
- 上游 crate 明确说明它假定桌面 Linux 系统遵循 Freedesktop 约定。

使用的源码依据：

- `trash-5.2.5/src/lib.rs`
- `trash-5.2.5/src/freedesktop.rs`

## Windows

上游后端通过 `IFileOperation` 调用 Windows Shell / Recycle Bin API，并启用了 undo。

实际影响：

- 删除目标会进入回收站，而不是直接 unlink。
- Windows 现在已经加入 CI 矩阵，但在第一轮 GitHub Actions 变绿之前，仍应把它视为“已配置、待验证”，而不是已经完成项目级实测。

使用的源码依据：

- `trash-5.2.5/src/windows.rs`

## 兼容性长期承诺

[rm-option-compatibility.md](rm-option-compatibility.md) 中定义的 CLI 兼容契约，目标是保持平台无关。
平台差异应该影响“如何进入系统回收站”，而不是影响参数解析规则。

如果未来某个平台必须出现 CLI 层面的特例，应该在发布前明确写入文档。
