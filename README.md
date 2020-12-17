# Honor
A fast and stable League of Legends client built with Rust 

*"Honor is the **Rust** on a dull blade"* ~ Sivir

Built with:
- rust
- druid
- tungstenite / async-tungstenite

On launch
- Call riotclient_killux
- If not connected, try to connect
- If can't connect try to relaunch

- try request
- if fail, return err result
- prompt modal, highlight Reconnect

- upon trying to connect, check if the LCU is running
- if LCU is not running, return err result
- prompt modal, highlight Restart LCU

| Reconnect | Restart LCU | Exit Honor |

- Upon sucessful restart, automatically try to reconnect
