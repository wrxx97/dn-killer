# DN Killer for Windows

GPUI 和 Rust 实现的 Windows x64 原生悬浮窗。可以同时运行多个倒计时、全局监听快捷键、播报提醒，以及用独立快捷键退出指定进程。

安装包和便携版 EXE 可从 [GitHub Releases](https://github.com/wrxx97/dn-killer/releases) 下载。

## 使用

- 首次启动有一项 15 秒的示例任务。松开 **F7** 或点击播放图标启动，剩 10 秒时播报。再次松开绑定键可重启对应任务。同一个键可绑定多项任务。
- 点击加号在弹窗中新建任务，点击铅笔编辑，点击铅笔右边的垃圾桶删除。新建和编辑弹窗按表单内容自动定高。任务弹窗点击「保存」后生效，点击「取消」或 X 可关闭草稿。时长填写整数秒，快捷键留空时可通过播放图标手动启动。
- 聚焦快捷键输入框后直接按下要设置的键。支持 F1–F12，也可用 Ctrl、Alt、Shift、Win 搭配功能键、字母或数字。进程快捷键在按下时触发，任务快捷键在主键松开时触发。
- 齿轮打开独立设置窗口。退出进程名与热键、语音开关和模板、提醒时间、背景不透明度和文字颜色的有效修改会自动保存并立即应用。模板支持 `{title}` 和 `{left}`。点窗口右上角 X 关闭设置。
- 设置、新建和编辑弹窗的背景和标题栏固定为 80% 透明。按住标题栏可拖动窗口。
- 齿轮右边的图钉切换主窗口置顶。实心表示已置顶，空心表示已取消置顶。
- 默认按下 **Alt+F4** 或点击「Alt+F4 退出」可结束 `DragonNest_x64.exe`。旧版配置中的默认 F6 会随新版切换到 Alt+F4。该组合在其他程序前台仍有 Windows 自带的关闭行为。窗口会显示退出结果或具体的 Windows 错误。清空退出快捷键后仍可使用按钮。
- 鼠标移到顶部标题区域会显示四向移动光标，按住即可拖动悬浮窗。点主窗口的 X 直接退出。最小化后可点击托盘图标恢复。每个 Windows 会话只能运行一个实例。
- 主窗口按实际任务列表和状态信息调整高度。列表超过可用高度时可滚动。

任务编辑后的「保存」和设置自动保存使用 `%LOCALAPPDATA%\dn-killer\config.json`。首次启动使用默认配置，运行中的倒计时不写入配置。

## 构建和安装包

需要 Rust 1.100 或以上的 MSVC 工具链，以及带 Windows SDK 的 Visual Studio C++ 构建工具。在本目录运行：

```powershell
cargo run --locked
cargo test --locked
```

使用 PowerShell 7 和 NSIS 3 生成独立的 Windows x64 安装包：

```powershell
pwsh -File .\installer\package.ps1
```

如果 `makensis.exe` 不在 PATH，可通过 `-MakeNsis` 指定完整路径。安装包输出到 `target\DN-Killer-<版本>-x64-setup.exe`。安装作用于当前用户，卸载会保留任务和配置。

GitHub Actions 会在拉取请求中验证格式和测试。手动运行可生成可下载的 Windows 构建产物。推送与 `Cargo.toml` 版本一致的 `v<版本>` 标签后，工作流会构建安装包和便携版 EXE，并附到对应的 GitHub Release。
