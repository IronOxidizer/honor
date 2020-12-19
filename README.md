# Honor
A fast and stable League of Legends client built with Rust 

*"Honor is the **Rust** on a dull blade"* ~ Sivir

Built with:
- rust
- druid
- async-tungstenite

On launch
- Call riotclient_killux

- try request
- if fail, return err result
- prompt modal, highlight Reconnect

- upon trying to connect, check if the LCU is running
- if LCU is not running, return err result
- prompt modal, highlight Restart LCU

| Reconnect LCU | Restart LCU |

- Upon sucessful restart, automatically try to reconnect

- On first occurence,
    - don't prompt user, do it automatically for automatic connecting on first launch
    - kill ux

https://127.0.0.1:50473/lol-summoner/v1/current-summoner


Consider auto reconnect on modal and only have one button to restart LCU, highlight button when LCU is not running and add "(recommended)" above it.

Use async rwlock over mutex

If waiting for response, set cursor to spinner?

Honor isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

Honor was created under Riot Games' "Legal Jibber Jabber" policy using assets owned by Riot Games.  Riot Games does not endorse or sponsor this project.
