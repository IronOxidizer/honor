
use std::process::Command;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use anyhow::Result;
use regex::Regex;
use base64;
use app_dirs::{data_root, AppDataType};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json;
use futures::future;
use async_std::{task, net::TcpStream};
use async_native_tls::TlsConnector;
use async_native_tls::TlsStream;
use async_tungstenite::client_async;
use async_tungstenite::tungstenite::handshake::client::Request;
use async_tungstenite::WebSocketStream;

/*
fn launch_client() -> Result<(), _> {
    //Check how mimic and decieve does this
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
}

fn connect_client() -> Result<(), _> {

}

fn is_honor_running() -> Bool {

}
*/

// https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#the-process-list-method
// Consider using the lockfile method once we have the riot client path
fn get_client_info() -> Result<(u16, String)> {
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
        Ok((port, token))
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

pub async fn connect_to_lcu() -> Result<WebSocketStream<TlsStream<TcpStream>>> {
    const HOST: &str = "127.0.0.1";
    let (port, token) = get_client_info()?;
    let stream = TcpStream::connect(format!("{}:{}", HOST, port)).await.unwrap();
    // LCU uses a self-signed certificate so create a custom connector to skip TLS verification
    //     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#performing-our-first-request
    //     Consider adding the root certificate in the future, more complicated and might cause other issues
    let tls_stream = TlsConnector::new().danger_accept_invalid_certs(true)
        .connect(HOST, stream).await.unwrap();

    // Set Basic authentication with request header, e.g riot:sesspswd => base64 encode => cmlvdDpwYXNzd29yZA==
    //     https://www.hextechdocs.dev/lol/lcuapi/6.getting-started-with-the-lcu-api#connecting
    let encoded_token = base64::encode(format!("riot:{}", token));
    eprintln!("{} | {}",token, encoded_token);
    let request = Request::get(format!("wss://{}:{}", HOST, port))
        .header("authorization", format!("Basic {}", encoded_token)).body(()).unwrap();
    let (ws_stream, _) = client_async(request, tls_stream)
        .await.expect("Could not connect");
    Ok(ws_stream)
}