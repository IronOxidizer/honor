pub mod riotclient;
pub mod lol_summoner;
pub mod lol_chat;
pub mod lol_game_queues;

use std::{sync::Arc,
    process::Command,
    path::PathBuf,
    fs::{File, read_to_string}};

use anyhow::Result;
use serde::{Deserialize, de::DeserializeOwned};
use app_dirs::{data_root, AppDataType};

use reqwest::Url;
use futures::{SinkExt, stream::{SplitSink, SplitStream}};
use tokio::{sync::Mutex, net::TcpStream};
use tokio_native_tls::TlsStream;
use tokio_tungstenite::{TlsConnector, WebSocketStream, stream::Stream, tungstenite::{self, Message, http::Request}};

use druid::{Data, Lens};

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

pub enum MessageTypes {
    Welcome = 0,
    Prefix = 1,
    Call = 2,
    CallResult = 3,
    CallError = 4,
    Subscribe = 5,
    Unsubscribe = 6,
    Publish = 7,
    Event = 8
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

pub fn run_lcu() -> Result<()> {
    if cfg!(target_os = "windows") {
        Command::new(get_riot_client_path()?)
            .args(&[r"C:\Riot Games\Riot Client\RiotClientServices.exe", "--launch-product=league_of_legends", "--launch-patchline=live"])
            .spawn()?;
    } else {
        // MacOS
        unimplemented!();
    }
    Ok(())
}

// https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#the-process-list-method
// Consider using the lockfile method once we have the riot client path
// This should be cached somewhere unless LCU is reset, can be expensive
pub fn get_lcu_connect_info() -> Result<(u16, String)> {
    if cfg!(target_os = "windows") {
        let path = get_riot_client_path()?.join("../../League of Legends/lockfile").canonicalize()?;
        let contents = read_to_string(path)?;
        let segments: Vec<&str> = contents.split(':').collect();
        let port: u16 = segments[2].parse()?;
        let token = segments[3];
        Ok((port, format!("Basic {}", base64::encode(format!("riot:{}", token)))))
    } else {
        // MacOS
        unimplemented!()
    }
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

// LCU uses a self-signed certificate so create a custom connector to skip TLS verification
//     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
//     Consider adding the root certificate in the future, more complicated and might cause other issues
//     Get cert from here https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
// Set Basic authentication with request header, e.g riot:sesspswd => base64 encode => cmlvdDpwYXNzd29yZA==
//     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#connecting
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

// Returns a generic that implements Deserialize
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