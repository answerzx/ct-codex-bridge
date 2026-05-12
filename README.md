# CT Codex Bridge

一个面向局域网的 Codex 账号切换 Web 面板。它读取 Cockpit Tools 保存的 Codex 账号数据，把选中的账号投影到本机 Codex App 使用的登录态文件和 macOS Keychain，然后重启 Codex App。

这不是 Cockpit Tools 的 Fork。它是一个新的独立 Web 项目，用来提供一个更轻量的局域网前端控制面板，只处理 Codex 账号切换。

## 功能

- 在局域网中提供 Web 面板，默认端口 `8787`
- 读取 CT 保存的 Codex 账号索引和账号详情
- 以卡片形式显示账号邮箱、套餐、当前账号状态、5 小时额度、周额度、上次使用时间
- 精确按 `accountId` 切换账号
- 写入 Codex 当前登录态：
  - `~/.codex/auth.json`
  - macOS Keychain service: `Codex Auth`
  - `~/.codex/.cockpit_codex_auth.json`
- 更新 CT 的 `current_account_id` 和目标账号 `last_used`
- 通过 `kill -15` 关闭 Codex App，再使用 `open -n -a /Applications/Codex.app` 重启
- 支持 macOS 用户级 LaunchAgent 后台运行，并在用户登录系统时自动启动
- 使用长期签名 Cookie 记住浏览器登录状态

## 不做什么

- 不管理 Antigravity 账号
- 不导入、删除或编辑 CT 账号
- 不刷新 token
- 不调用 OpenAI 账号检测或额度刷新接口
- 不校验账号是否仍然可用
- 不把 token、API key、refresh token 返回给前端

额度信息只展示 CT 已经保存到本地账号详情文件里的数据。如果 CT 没有更新额度，本项目不会主动联网刷新。

## 数据来源

本项目读取 CT 的本地 Codex 账号数据：

```text
~/.antigravity_cockpit/codex_accounts.json
~/.antigravity_cockpit/codex_accounts/{accountId}.json
```

账号详情文件中可能包含 OAuth token 或 API key。本项目后端会读取这些字段用于切换账号，但 API 响应和前端页面不会返回这些敏感字段。

## 切换逻辑

切换账号时，后端执行以下步骤：

1. 按精确 `accountId` 加载 CT 保存的账号详情
2. 根据账号类型生成 Codex `auth.json`
3. 写入 `~/.codex/auth.json`
4. 写入 macOS Keychain，service 为 `Codex Auth`
5. 写入 `~/.codex/.cockpit_codex_auth.json`
6. 更新 CT 账号索引中的 `current_account_id`
7. 更新目标账号详情的 `last_used`
8. 关闭并重新打开 Codex App

切换过程会做本地备份；如果重启 Codex App 等步骤失败，会尽量回滚已经写入的文件状态。

## 安装与运行

构建：

```bash
cargo build --release
```

初始化或更新面板访问密码：

```bash
./target/release/ct-codex-bridge setup-password
```

手动运行：

```bash
./target/release/ct-codex-bridge serve
```

访问：

```text
http://<mac-lan-ip>:8787
```

安装为 macOS 用户级后台服务：

```bash
./target/release/ct-codex-bridge install-launch-agent
```

卸载后台服务：

```bash
./target/release/ct-codex-bridge uninstall-launch-agent
```

查看路径和状态：

```bash
./target/release/ct-codex-bridge doctor
```

## 本地配置

本项目自己的配置文件位于：

```text
~/.ct-codex-bridge/config.json
```

其中：

- `password_hash` 是面板访问密码的 Argon2 哈希，不是明文密码
- `session_secret` 用来验证浏览器长期登录 Cookie

后台服务安装后会使用：

```text
~/.ct-codex-bridge/bin/ct-codex-bridge
~/Library/LaunchAgents/com.ct-codex-bridge.plist
~/.ct-codex-bridge/logs/
```

## 隐私注意事项

这个项目会在本机读取和写入敏感登录态。请只在你信任的 Mac 和局域网环境中运行。

不要提交或公开以下文件和目录：

```text
~/.ct-codex-bridge/config.json
~/.ct-codex-bridge/logs/
~/.ct-codex-bridge/backups/
~/.antigravity_cockpit/
~/.codex/
```

这些路径可能包含：

- 面板密码哈希
- Cookie 签名密钥
- CT 保存的 Codex token
- OpenAI API key
- Codex 当前登录态
- 切换前后的备份文件
- 本机路径和运行日志

仓库中的 `.gitignore` 已排除常见本地运行状态和隐私文件。提交前仍建议运行：

```bash
git status --short
rg -n "access_token|refresh_token|id_token|OPENAI_API_KEY|session_secret|password_hash|sk-" . --glob '!target/**' --glob '!.git/**'
```

测试代码中出现的 token 或 API key 字符串是假的测试夹具，不是真实凭据。

## 平台要求

- macOS
- Rust toolchain
- Codex App 安装在 `/Applications/Codex.app`
- Cockpit Tools 已保存 Codex 账号数据

## 许可证

MIT
