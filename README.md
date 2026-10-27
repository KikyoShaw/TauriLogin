# TauriLogin

基于 **Tauri v2 + Vue 3 + TypeScript** 的微信 / QQ 网页版扫码登录 Demo：在独立小窗口中加载官方扫码页，扫码确认后由 Rust 侧拦截回调导航，提取 `code` / `state` 回传主窗口展示，验证 OAuth 扫码登录的完整链路（不对接后端，仅演示拿授权码）。

## 功能

- 微信开放平台扫码登录（`open.weixin.qq.com/connect/qrconnect`，`snsapi_login`）
- QQ 互联扫码登录（`graph.qq.com/oauth2.0/show`，`display=pc`）
- 自填 AppID / Redirect URI / State（自动生成 + 手动重新生成），适配任意开放平台账号
- 回调拦截：扫码确认后 **不会真正跳转** redirect_uri，`code` / `state` / 原始查询串直接展示在主窗口，code 一键复制
- State 一致性校验（防回调伪造演示）
- 拦截 `weixin://`、`mqqapi://` 等拉起本地客户端的非网页跳转，并拦截页面内 `window.open` 弹窗
- 用户未扫码关闭窗口时发出取消事件；微信拒绝授权（`code=100xx`）标记为错误回调
- 事件日志面板：实时显示登录窗口的导航与拦截记录

## 技术要点

- 登录窗口由 Rust 侧运行时创建（`WebviewWindowBuilder`），`on_navigation` 闭包捕获 `redirect_uri` 做匹配拦截（scheme + host + port + path 比较），返回 `false` 阻止导航
- 关闭登录窗口在独立线程执行，规避 WebView2 `NavigationStarting` 回调内销毁 webview 的重入风险
- 登录窗口不匹配任何 Tauri capability，官方页面在 webview 内 **零 IPC 权限**；Rust → 前端的事件推送（`oauth://result` / `oauth://log` / `oauth://cancelled`）不受权限门控

## 使用

前置要求：[Rust](https://rustup.rs/)、Node.js ≥ 18、Windows（WebView2）。

```bash
npm install
npm run tauri dev
```

1. 选择「微信登录」或「QQ 登录」tab
2. 填入你在 [微信开放平台](https://open.weixin.qq.com/) / [QQ 互联](https://connect.qq.com/) 创建的应用的 AppID
3. Redirect URI 填一个与后台「授权回调域」一致的回调地址（无需真实存在，扫码后由本应用拦截，不会发起请求）
4. 点击「开始扫码登录」，在弹出的窗口中用手机扫码并确认
5. 登录窗口自动关闭，主窗口展示捕获的 `code` / `state`

> 生产环境中，`code` 需提交到你的后端，由后端携带 `secret` 向微信 / QQ 换取 `access_token`（换取步骤必须在服务端完成，`secret` 不能出现在客户端）。

## 项目结构

```
src/
  App.vue                    # 外壳：tab 切换、结果/日志/期望 state 状态提升
  components/LoginPanel.vue  # 登录表单 + 事件订阅
  components/ResultPanel.vue # 拦截结果展示（code/state/一致性校验）
  components/EventLogPanel.vue
  services/login.ts          # invoke / listen 封装
  types/login.ts             # 前后端共享 DTO
src-tauri/src/
  lib.rs                     # 命令注册
  login.rs                   # 核心：扫码页 URL 构造、回调匹配、登录窗口与导航拦截
```

## License

[Apache-2.0](LICENSE)
