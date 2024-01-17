#[derive(Default, Debug)]
pub struct SignalrConn {
    negociated: bool,
    cookie: String,
    negociate_response: Option<NegociateResponse>,
}

impl SignalrConn {
    pub fn negotiate() -> SignalrConn {
        let url = reqwest::Url::parse_with_params(
            NEGOTITE_URL,
            &[
                ("connectionData", CONNECTION_DATA),
                ("clientProtocol", CLIENT_PROTOCOL),
            ],
        )
        .unwrap();
        let response = reqwest::blocking::get(url).unwrap();
        let cookie = response
            .headers()
            .get("Set-Cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let negociate_response =
            serde_json::from_str::<NegociateResponse>(&response.text().unwrap()).ok();

        SignalrConn {
            cookie,
            negociated: true,
            negociate_response,
        }
    }
}

#[derive(Default, Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct NegociateResponse {
    url: String,
    connection_token: String,
    connection_id: String,
    keep_alive_timeout: f32,
    disconnect_timeout: f32,
    connection_timeout: f32,
    try_web_sockets: bool,
    protocol_version: String,
    transport_connect_timeout: f32,
    long_poll_delay: f32,
}

const NEGOTITE_URL: &str = "https://livetiming.formula1.com/signalr/negotiate";
const CLIENT_PROTOCOL: &str = "1.5";
const CONNECTION_DATA: &str = "[{\"name\":\"Streaming\"}]";
