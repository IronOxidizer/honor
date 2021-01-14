pub mod riotclient;
pub mod summoner;
pub mod chat;
pub mod game_queues;
pub mod lobby;

use std::{sync::Arc,
    process::Command,
    path::PathBuf,
    io::{Error, ErrorKind},
    fs::{File, read_to_string}};

use serde_json;
use anyhow::Result;
use serde::{Deserialize, de::DeserializeOwned};
use app_dirs::{data_root, AppDataType};

use reqwest::{Url, header};
use futures::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use tokio::{sync::Mutex, net::TcpStream};
use tokio_native_tls::TlsStream;
use tokio_tungstenite::{TlsConnector, WebSocketStream, stream::Stream, tungstenite::{self, Message, http::Request}};

use druid::{Data, Lens, Selector, SingleUse, Target, ExtEventSink};

// Use the assumption that LCU is stable and wont crash,
//  Seperate sink and stream on init so the listener can take sole ownership of sink
//  and stream can be shared everywhere else wrapped in Arc
// If LCU turns out to be unstable and disconnects requiring a new connection, might need to add WSS to AppState so sink and stream can be updated
//  on reconnect
// If LCU is REALLY unstable, add connection info to AppState so client can connect to LCU when LCU restarts with new connection info.

pub type WampSink = Arc<Mutex<SplitSink<WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>, Message>>>;
pub type WampStream = SplitStream<WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>>;

#[derive(Clone, Data, Lens)]
pub struct HttpConnection {
    pub client: Arc<reqwest::Client>,
    pub port: u16,
    pub token: String
}

pub const POST_INVITE: Selector<SingleUse<u64>> = Selector::new("POST_INVITE");
pub const SET_CURRENT_SUMMONER: Selector<SingleUse<Arc<summoner::Summoner>>> = Selector::new("SET_CURRENT_SUMMONER");
pub const SET_QUEUES: Selector<SingleUse<Arc<game_queues::Queues>>> = Selector::new("SET_QUEUES");
pub const SET_FRIENDS: Selector<SingleUse<chat::Friends>> = Selector::new("SET_FRIENDS");
pub const SET_LOBBY: Selector<SingleUse<Arc<lobby::Lobby>>> = Selector::new("UPDATE_LOBBY");
pub const UPDATE_FRIEND: Selector<SingleUse<chat::Friend>> = Selector::new("UPDATE_FRIEND");

pub enum MessageTypes {
    // Welcome = 0,
    // Prefix = 1,
    // Call = 2,
    // CallResult = 3,
    // CallError = 4,
    Subscribe = 5,
    // Unsubscribe = 6,
    // Publish = 7,
    // Event = 8
}

// TODO: Use lazy static, this shouldn't change
fn get_riot_client_path() -> Result<PathBuf> {
    #[derive(Deserialize)]
    struct RiotClientInstalls {rc_live: PathBuf}

    let mut installs_path = data_root(AppDataType::SharedData)?;
    installs_path.push("Riot Games/RiotClientInstalls.json");
    let installs_data: RiotClientInstalls = serde_json::from_reader(File::open(&installs_path)?)?;
    let rc_live = installs_data.rc_live;
    Ok(rc_live)
}

pub fn start_lcu() -> Result<()> {
    Command::new(get_riot_client_path()?)
        .args(&["--launch-product=league_of_legends", "--launch-patchline=live"])
        .spawn()?;
    Ok(())
}

// https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#the-process-list-method
// Consider using the lockfile method once we have the riot client path
// This should be cached somewhere unless LCU is reset, can be expensive
pub fn get_lcu_connect_info() -> Result<(u16, String)> {
    let path = get_riot_client_path()?.join("../../League of Legends/lockfile").canonicalize()?;
    let contents = read_to_string(path)?;
    let segments: Vec<&str> = contents.split(':').collect();
    let port: u16 = segments.get(2)
        .ok_or(Error::new(ErrorKind::NotFound, "Lockfile port nonexistent"))?.parse()?;
    let token = segments.get(3)
        .ok_or(Error::new(ErrorKind::NotFound, "Lockfile token nonexistent"))?;
    Ok((port, format!("Basic {}", base64::encode(format!("riot:{}", token)))))
}

pub fn get_connection(port: u16, token: String) -> Result<HttpConnection> {
    Ok(HttpConnection {
        // Consider adding the cert using client.add_root_certificate(cert) 
        // Get cert from here: https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
        client: Arc::new(reqwest::Client::builder().danger_accept_invalid_certs(true)
            .build().expect("Could not build client")),
        port: port,
        token: token
    })
}

// Set Basic authentication with request header, e.g riot:sesspswd => base64 encode => cmlvdDpwYXNzd29yZA==
//     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#connecting
// LCU uses a self-signed certificate so create a custom connector to skip TLS verification
//     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
//     Consider adding the root certificate in the future, more complicated and might cause other issues
//     Get cert from here https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
pub async fn connect_lcu_wamp(port: u16, token: String) -> Result<WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>> { // Result<WebSocketStream<TlsStream<TcpStream>>> {
    const HOST: &str = "127.0.0.1";
    let request = Request::get(format!("wss://{}:{}", HOST, port))
        .header("authorization", token).body(())?;
    let stream = TcpStream::connect(format!("{}:{}", HOST, port)).await?;
    let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(true).build()?;
    
    let (wss, _) = tokio_tungstenite::client_async_tls_with_config(request, stream, None, Some(tls_connector)).await?;
    Ok(wss)
}

pub fn wamp_send(wamp_sink: WampSink, message_type: MessageTypes, message: &'static str) {
    let json = format!("[{}, \"{}\"]", message_type as u8, message);
    eprintln!("Sending: {}", json);
    tokio::spawn( 
        async move {
        wamp_sink.lock().await.send(tungstenite::Message::from(
            json
        )).await.expect("WAMP Sink failed to send")
    });
}

// Returns a generic that implements DeserializeOwned
// Reuse client everywhere for higher perofrmance. reqwest::get creates a new client each time which is slow
pub async fn get_request<T>(connection: HttpConnection, endpoint: &str) -> Result<T> 
where T: DeserializeOwned {
    let url = Url::parse(format!("https://{}:{}/{}", super::HOST, connection.port, endpoint).as_str())?;

    // eprintln!("Request: {:?}\n\n", url.clone());
    // let res = connection.client.get(url)
    //     .header("authorization", connection.token)
    //     .send().await?;
    // eprintln!("Response: {:?}", res.text().await?);
    // todo!()

    Ok(connection.client.get(url)
        .header("authorization", connection.token)
        .send().await?.json().await?)
}

pub async fn post_request(connection: HttpConnection, endpoint: &str, payload: String) -> Result<reqwest::Response> {
    let url = Url::parse(format!("https://{}:{}/{}", super::HOST, connection.port, endpoint).as_str())?;

    //eprintln!("{}", payload);

    Ok(connection.client.post(url)
        .body(payload)
        .header(header::AUTHORIZATION, connection.token)
        .header(header::CONTENT_TYPE, "application/json")
        .send().await?)
}

pub async fn wamp_poll_spin(mut wamp_stream: WampStream, event_sink: ExtEventSink) {
    loop {
        // Could be much much cleaner if we could .unwrap_or(|_|continue)
        let message = if let Some(Ok( m)) = wamp_stream.next().await { m } else { continue };
        let json = if let Ok(j) = serde_json::from_slice::<serde_json::Value>(message.into_data().as_slice())
            { j } else { continue };
        let json_arr = if let Some(ja) = json.as_array() {
            if ja.len() < 3 { continue } else { ja }
        } else { continue };
        let event = if let Some(es) = json_arr[1].as_str() { es } else { continue };
        
        match event {
            "OnJsonApiEvent_lol-chat_v1_friends" => {
                match serde_json::from_value::<chat::Friend>(json[2]["data"].clone()) {
                    Ok(friend) => {
                        event_sink.submit_command(
                            UPDATE_FRIEND,
                            SingleUse::new(friend),
                            Target::Auto).unwrap();
                    } Err(e) => eprintln!("\n\nUncaptured friend event: {:?}\n{:?}", e, json[2]["data"].clone())
                }
            }, "OnJsonApiEvent_lol-lobby_v2_lobby" => {
                match serde_json::from_value::<Option<lobby::Lobby>>(json[2]["data"].clone()) {
                    Ok(Some(lobby)) => {
                        event_sink.submit_command(
                            SET_LOBBY,
                            SingleUse::new(Arc::new(lobby)),
                            Target::Auto).unwrap();
                    } Ok(None) => {
                        event_sink.submit_command(
                            SET_LOBBY,
                            SingleUse::new(Arc::new(lobby::Lobby::default())),
                            Target::Auto).unwrap();
                    } Err(e) => eprintln!("\n\nUncaptured lobby event: {:?}", e)
                }
            }, _ => eprintln!("Event no match: {}", event)
        }
    }
}