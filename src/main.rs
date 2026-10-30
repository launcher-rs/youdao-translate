use arboard::Clipboard;
use launcher_plugin::jsonrpc::{
    JsonRpcAction, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseDataBuilder, Method,
};
use std::env;
use youdao::{YouDaoClient, YouDaoConfig};

const ICON_PATH: &str = "Images\\icon.png";

fn main() {
    let argument = env::args().nth(1).unwrap();
    let request = JsonRpcRequest::from(&argument).unwrap();
    let input = request.parameter_or_default(0);

    match &request.method {
        Method::Query => {
            let settings = &request.settings;
            let app_secret = settings
                .get("app_secret")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            if app_secret.is_empty() {
                request
                    .set_simple_response(
                        "请设置app_secret",
                        "请设置app_secret",
                        ICON_PATH,
                        Some(JsonRpcAction::open_setting_dialog()),
                    )
                    .unwrap();
                return;
            }

            let app_key = settings
                .get("app_key")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            if app_key.is_empty() {
                request
                    .set_simple_response(
                        "请设置app_key",
                        "请设置app_key",
                        ICON_PATH,
                        Some(JsonRpcAction::open_setting_dialog()),
                    )
                    .unwrap();
                return;
            }

            let default_language = settings
                .get("default_language")
                .and_then(|value| value.as_str())
                .unwrap_or("zh-CHS")
                .trim()
                .to_string();

            let mut query_data = JsonRpcResponse::new();

            if !input.is_empty() {
                let mut config = YouDaoConfig::new(&app_key, &app_secret);
                config.set_to(&default_language);
                config.set_from("auto");

                let client = YouDaoClient::new(config);
                match client.text_translate(&input) {
                    Ok(translate_result) => {
                        if let Some(translate_result) = translate_result.translation {
                            let translate_result = translate_result.join("");
                            let sub_tile = format!("{}  回车复制", input);
                            let title = format!("译文: {}", translate_result);
                            let item = JsonRpcResponseDataBuilder::new()
                                .title(title)
                                .sub_title(sub_tile)
                                .icon_path(ICON_PATH)
                                .json_rpcaction(Some(JsonRpcAction::custom(
                                    "copy".to_string(),
                                    vec![translate_result],
                                )))
                                .build();

                            query_data.add_data(item);
                            request.set_response(query_data).unwrap();
                        } else {
                            request
                                .set_simple_response(
                                    "发生错误",
                                    &translate_result.error_code,
                                    ICON_PATH,
                                    None,
                                )
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        request
                            .set_simple_response("发生错误", &err.to_string(), ICON_PATH, None)
                            .unwrap();
                    }
                }
            } else {
                request
                    .set_simple_response("翻译", "请输入要翻译的内容", ICON_PATH, None)
                    .unwrap();
            }
        }

        Method::ContextMenu => {
            let mut context_menu_data = JsonRpcResponse::new();

            let item = JsonRpcResponseDataBuilder::new()
                .title("打开设置对话框".to_string())
                .sub_title("打开设置对话框".to_string())
                .icon_path(ICON_PATH)
                .json_rpcaction(Some(JsonRpcAction::open_setting_dialog()))
                .build();
            context_menu_data.add_data(item);

            request.set_response(context_menu_data).unwrap()
        }
        Method::Custom(method) if method == "copy" => {
            let content = request.parameter_or_default(0);
            if !content.is_empty() {
                // 复制到剪切板
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(content).unwrap();
            }
        }
        _ => {}
    }
}
