# Changelog

All notable changes to this project will be documented in this file.

## [0.0.11] - 2026-04-07

### Fixed
- **修复 Windows 兼容性** - 使用跨平台的 `Path::file_name()` 替代硬编码 `/` 分割路径
  - 之前在 Windows 上生成的章节名称会是完整路径而不是文件名
  - 现在在所有平台上都能正确提取文件名

### CI
- **优化 CI 配置** - Clippy 只在 Ubuntu 运行，单元测试在 Ubuntu/Windows/macOS 三平台运行
  - 减少不必要的 CI 资源消耗
  - 确保跨平台兼容性验证

## [0.0.10] - 2026-04-07

### Fixed
- 自动忽略输出文件，避免 SUMMARY.md 被包含在生成结果中
- 支持输出到任意位置（src/SUMMARY.md、./SUMMARY.md、docs/SUMMARY.md 等）

## [0.0.9] - 2026-04-07

### Fixed
- 修复 URL 过度编码问题
  - mdbook 的 SUMMARY.md 使用文件系统路径而非 URL
  - 之前的完整 URL 编码导致文件名过长错误
  - 现在只转义空格字符，支持 Unicode 文件名

## [0.0.8] - 2026-04-07

### Fixed
- **重大修复**: 生成的 `SUMMARY.md` 现在使用相对路径而非绝对路径
  - 修复了 mdbook 无法解析绝对路径导致无法启动的问题
  - 路径现在相对于指定的源目录（`--dir` 参数）

- **重大修复**: 文件名中的特殊字符现在会进行 URL 编码
  - 修复了文件名包含空格时 mdbook 解析失败的问题
  - 例如：`file with space.md` 生成链接 `file%20with%20space.md`
  - 保留安全字符：`-` `_` `.` `~` `/`

### Changed
- 添加 `percent-encoding` 依赖用于 URL 编码
- `SummaryItem` 结构体现在同时保存相对路径和绝对路径

## [0.0.7] - 2024-01-01

### Added
- 初始发布版本
- 支持生成 `SUMMARY.md` 文件
- 支持 `.gmsignore` 忽略文件（使用 `.gitignore` 语法）
- 支持自动检测 `README.md` 作为章节介绍
- 支持按字母顺序排序章节
