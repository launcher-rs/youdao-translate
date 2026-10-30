# 有道翻译插件

一个使用 Rust 编写的搜索启动器插件。输入文本后调用有道翻译 API，并可通过回车将译文复制到剪贴板。

## 配置

在启动器的插件设置页填写有道 API 的 `app_key` 和 `app_secret`，并选择默认目标语言。密钥使用密码框输入；请勿将真实密钥提交到代码仓库或发布包。

## 使用

1. 安装插件并在插件设置中填写 API 信息。
2. 在搜索框输入触发词 `yd`，后接要翻译的文本。
3. 选择翻译结果并按回车复制译文。

## 开发

依赖 `launcher-plugin 0.2.0`：

```toml
launcher-plugin = "0.2.0"
```

插件不在进程中自行休眠。清单声明 `SearchDelayTime: 500`，由 Ruseek 在启动插件前等待 500 毫秒；宿主会将 JSON-RPC 请求作为第一个命令行参数传入，插件把 JSON 结果写入标准输出。

```bash
cargo dev
```

开发命令会编译并复制插件到测试目录。可通过 `PLUGIN_DIR` 指定目标插件目录。

## 打包

```bash
cargo dabao
```

打包文件输出到 `dist`。压缩包包括可执行文件、`plugin.json`、`SettingsTemplate.yaml` 和图标资源。

## SDK

插件使用 [launcher-plugin](https://crates.io/crates/launcher-plugin) 处理 JSON-RPC 请求与响应。该库提供安全的参数读取方法、响应 builder，以及常用动作构造器。
