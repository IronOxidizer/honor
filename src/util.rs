
use std::process::Command;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use serde_json;
use anyhow::Result;
use serde::{Deserialize, de::DeserializeOwned};
use reqwest::Url;
use druid::{Data, Lens, ExtEventSink};
use regex::Regex;
use base64;
use app_dirs::{data_root, AppDataType};

use super::lcu_api::*;
use super::views::AppView;

// Everything in an Rc up to the scope of which the data is accquired / changing
// When using Vector, Arc is not needed
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub event_sink: Arc<ExtEventSink>,
    pub connection: Connection,
    pub view: AppView,
    pub current_summoner: Arc<lol_summoner::Summoner>,
    pub queues: lol_game_queues::Queues
}

impl AppState {
    pub fn new(event_sink: Arc<ExtEventSink>, connection: Connection) -> Self {
        Self {
            event_sink: event_sink,
            connection: connection,
            view: Default::default(),
            current_summoner: Default::default(),
            queues: Default::default()
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Connection {
    pub client: Arc<reqwest::Client>,
    pub port: u16,
    pub token: String
}

pub fn get_connection_data() -> Result<Connection> {
    let (port, token) = get_lcu_info()?;
    Ok(Connection {
        // Consider adding the cert using client.add_root_certificate(cert) 
        // Get cert from here: https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
        client: Arc::new(reqwest::Client::builder().danger_accept_invalid_certs(true)
            .build().expect("Could not build client")),
        port: port,
        token: token
    })
}

/*
fn connect_client() -> Result<(), _> {

}

fn is_honor_running() -> Bool {

}
*/

// https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#the-process-list-method
// Consider using the lockfile method once we have the riot client path
// This should be cached somewhere unless LCU is reset, can be expensive
pub fn get_lcu_info() -> Result<(u16, String)> {
    if cfg!(target_os = "windows") {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
            --remoting-auth-token=([\w\-_]*)
            (?:.*)
            --app-port=(\d*)").unwrap();
        }

        let output = String::from_utf8(Command::new("wmic")
            .args(&["PROCESS", "WHERE", "name='LeagueClientUx.exe'"])
            .output()?.stdout)?;
        let captures = RE.captures(output.as_str()).ok_or(
            Error::new(ErrorKind::NotFound, "Could not parse client process"))?;
        let token = captures.get(1).ok_or(
            Error::new(ErrorKind::NotFound, "Could not parse client process")
        )?.as_str().to_string();
        let port = captures.get(2).ok_or(
            Error::new(ErrorKind::NotFound, "Could not parse client process")
        )?.as_str().parse::<u16>()?;
        Ok((port, format!("Basic {}", base64::encode(format!("riot:{}", token)))))
    } else {
        // MacOS
        unimplemented!()
    }
}

// TODO: Use lazy static, this shouldn't change
fn get_riot_client_path() -> Result<PathBuf> {
    #[derive(Deserialize)]
    struct RiotClientInstalls {rc_live: PathBuf}

    let mut installs_path = data_root(AppDataType::SharedData)?;
    installs_path.push("Riot Games/RiotClientInstalls.json");
    let reader = BufReader::new(File::open(&installs_path)?);
    let installs_data: RiotClientInstalls = serde_json::from_reader(reader)?;
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

// pub async fn connect_to_lcu() -> Result<WebSocketStream<TlsStream<TcpStream>>> {
//     const HOST: &str = "127.0.0.1";
//     let (port, token) = get_lcu_info()?;
//     let stream = TcpStream::connect(format!("{}:{}", HOST, port)).await.unwrap();
//     // LCU uses a self-signed certificate so create a custom connector to skip TLS verification
//     //     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
//     //     Consider adding the root certificate in the future, more complicated and might cause other issues
//     //     Get cert from here https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
//     let tls_stream = TlsConnector::new().danger_accept_invalid_certs(true)
//         .connect(HOST, stream).await.unwrap();

//     // Set Basic authentication with request header, e.g riot:sesspswd => base64 encode => cmlvdDpwYXNzd29yZA==
//     //     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#connecting
//     let request = Request::get(format!("wss://{}:{}", HOST, port))
//         .header("authorization", token).body(()).unwrap();
//     let (ws_stream, _) = client_async(request, tls_stream).await?;
//     Ok(ws_stream)
// }

// Returns a generic that implements Deserialize
// Reuse client everywhere for higher perofrmance. reqwest::get creates a new client each time which is slow
pub async fn get_request<T>(connection: Connection, endpoint: &str) -> Result<T> 
where T: DeserializeOwned {
    let url = Url::parse(format!("https://{}:{}/{}", super::HOST, connection.port, endpoint).as_str())?;

    // eprintln!("Request: {:?}\n\n", url.clone());
    // let res = connection_data.client.get(url)
    //     .header("authorization", connection_data.token)
    //     .send().await?;
    // println!("Response: {:?}", res.text().await?);
    // todo!()

    Ok(connection.client.get(url)
        .header("authorization", connection.token)
        .send().await?.json().await?)
}