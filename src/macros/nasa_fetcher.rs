// #[macro_export]
// macro_rules! reqwester {
//     ($url:expr, $val:expr) => {{
//         let http_to_url = reqwest::Url::parse($url).unwrap();
//
//         let http_response_text = http_to_url.text().await.unwrap_or_else(|_| "No text fetched, sorry...".to_string());
//
//         let extract_text_from_api: Value = serde_json::from_str(&http_response_text).expect("Unable to format JSON.");
//
//         if let Some(value) = extract_text_from_api.get($val) {
//             value.to_string()
//         } else {
//             "No value found for the given key.".to_string()
//         }
//     }}
// }