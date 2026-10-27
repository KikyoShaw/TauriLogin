use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Emitter, Manager, Url, WebviewUrl, WebviewWindowBuilder, WindowEvent,
    webview::NewWindowResponse,
};

/// 登录窗口标签。同时只允许一个登录窗口，重复发起时聚焦已有窗口。
pub const LOGIN_WINDOW_LABEL: &str = "login";

// 若微信扫码页对 WebView2 默认 UA 有异议，取消下面注释启用桌面 Edge UA：
// const FALLBACK_EDGE_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginParams {
    /// "wechat" | "qq"
    pub provider: String,
    pub app_id: String,
    pub redirect_uri: String,
    pub state: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthResult {
    pub provider: String,
    pub code: Option<String>,
    pub state: Option<String>,
    pub raw_query: String,
    pub is_error: bool,
    pub url: String,
}

fn query_get(url: &Url, key: &str) -> Option<String> {
    url.query_pairs()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.to_string())
}

pub fn build_login_url(p: &LoginParams) -> Result<Url, String> {
    let base = match p.provider.as_str() {
        "wechat" => "https://open.weixin.qq.com/connect/qrconnect",
        "qq" => "https://graph.qq.com/oauth2.0/show",
        other => return Err(format!("未知的登录方式: {other}")),
    };
    let mut url = Url::parse(base).map_err(|e| format!("URL 构造失败: {e}"))?;
    {
        let mut q = url.query_pairs_mut();
        match p.provider.as_str() {
            "wechat" => {
                q.append_pair("appid", &p.app_id);
                // append_pair 会做百分号编码，前端传入原始 URL 即可，不要预编码
                q.append_pair("redirect_uri", &p.redirect_uri);
                q.append_pair("response_type", "code");
                q.append_pair("scope", "snsapi_login");
                q.append_pair("state", &p.state);
            }
            _ => {
                q.append_pair("which", "Login");
                q.append_pair("display", "pc");
                q.append_pair("response_type", "code");
                q.append_pair("client_id", &p.app_id);
                q.append_pair("redirect_uri", &p.redirect_uri);
                q.append_pair("state", &p.state);
            }
        }
    }
    if p.provider == "wechat" {
        // 微信要求 qrconnect URL 以 #wechat_redirect 结尾
        url.set_fragment(Some("wechat_redirect"));
    }
    Ok(url)
}

/// scheme + host + port + path 比较（忽略 redirect_uri 自带的 query，容忍尾斜杠差异）
fn matches_redirect(redirect: &Url, url: &Url) -> bool {
    fn norm_path(p: &str) -> &str {
        let p = p.trim_end_matches('/');
        if p.is_empty() { "/" } else { p }
    }
    redirect.scheme() == url.scheme()
        && redirect.host_str() == url.host_str()
        && redirect.port() == url.port()
        && norm_path(redirect.path()) == norm_path(url.path())
}

pub fn open_login(app: &AppHandle, params: LoginParams) -> Result<(), String> {
    if params.app_id.trim().is_empty() {
        return Err("appid 不能为空".into());
    }
    if params.redirect_uri.trim().is_empty() {
        return Err("redirect_uri 不能为空".into());
    }
    if let Some(existing) = app.get_webview_window(LOGIN_WINDOW_LABEL) {
        let _ = existing.set_focus();
        return Ok(());
    }

    let url = build_login_url(&params)?;
    let redirect = Url::parse(&params.redirect_uri)
        .map_err(|e| format!("redirect_uri 不是合法 URL: {e}"))?;

    let title = if params.provider == "wechat" {
        "微信扫码登录"
    } else {
        "QQ 扫码登录"
    };

    // on_navigation 与 on_new_window 各自捕获一份；thread::spawn 再用一份
    let nav_app = app.clone();
    let new_win_app = app.clone();
    let close_app = app.clone();
    let provider = params.provider.clone();

    // Destroyed 事件到来时是否已成功捕获回调；未捕获即视为用户取消
    let captured = Arc::new(AtomicBool::new(false));
    let captured_on_destroy = captured.clone();

    let window = WebviewWindowBuilder::new(app, LOGIN_WINDOW_LABEL, WebviewUrl::External(url))
        .title(title)
        .inner_size(420.0, 560.0)
        .resizable(false)
        .maximizable(false)
        .center()
        // .user_agent(FALLBACK_EDGE_UA)
        .on_new_window(move |url, _| {
            // 扫码页内的 window.open（ptlogin2 类流程等）一律拒绝，只记日志
            let _ = new_win_app.emit("oauth://log", format!("已拦截新窗口请求: {url}"));
            NewWindowResponse::Deny
        })
        .on_navigation(move |url| {
            let scheme = url.scheme();
            if scheme != "http" && scheme != "https" {
                // weixin://、mqqapi:// 等拉起本地客户端的跳转直接拦截
                let _ = nav_app.emit("oauth://log", format!("已拦截非网页跳转: {url}"));
                return false;
            }
            if matches_redirect(&redirect, url) {
                captured.store(true, Ordering::SeqCst);
                let code = query_get(url, "code");
                // 微信拒绝授权等错误以 code=100xx 形式回传；OAuth 标准则用 error 参数
                let is_error = code
                    .as_deref()
                    .is_some_and(|c| c.len() == 5 && c.starts_with("100"))
                    || query_get(url, "error").is_some();
                let _ = nav_app.emit(
                    "oauth://result",
                    OAuthResult {
                        provider: provider.clone(),
                        code,
                        state: query_get(url, "state"),
                        raw_query: url.query().unwrap_or("").to_string(),
                        is_error,
                        url: url.to_string(),
                    },
                );
                // 不能在 NavigationStarting 回调线程内销毁 webview（重入风险），另起线程关闭
                let thread_app = nav_app.clone();
                std::thread::spawn(move || {
                    if let Some(w) = thread_app.get_webview_window(LOGIN_WINDOW_LABEL) {
                        let _ = w.close();
                    }
                });
                return false;
            }
            let _ = nav_app.emit("oauth://log", format!("页面跳转: {url}"));
            true
        })
        .build()
        .map_err(|e| format!("创建登录窗口失败: {e}"))?;

    window.on_window_event(move |event| {
        if matches!(event, WindowEvent::Destroyed) && !captured_on_destroy.load(Ordering::SeqCst) {
            let _ = close_app.emit("oauth://cancelled", ());
        }
    });
    Ok(())
}

// 备注：当前微信/QQ 扫码页确认后均为顶层 location 跳转，on_navigation 可靠拦截。
// 若将来某提供商改为 form POST 等不触发导航的方式，降级方案为 on_page_load
// 配合 webview.url() 检查，在此模块内实现即可。
