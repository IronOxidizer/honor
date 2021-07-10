# Honor

![demo](https://user-images.githubusercontent.com/60191958/106922789-66976180-66db-11eb-8b0b-98d92d7d539d.png)

A fast and stable League of Legends matchmaking client built with Rust 

*"Honor is the **Rust** on a dull blade"* ~ Sivir

## Features

- Fast
- Stable
- Simple design
- Minimal CPU / RAM / storage usage
- Scalable from 640x360 to any size

## Built with

- [rust](https://www.rust-lang.org/)
- [druid](https://github.com/linebender/druid)

## Legal

Honor is licensed under AGPLv3+

Honor has been officially audited and registered under Riot's community development program. As such, it's been verified to operate within Riot's terms and conditions at no risk to its users.

Honor isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

Honor was created under Riot Games' "Legal Jibber Jabber" policy using assets owned by Riot Games.  Riot Games does not endorse or sponsor this project.

![honor-verification](https://user-images.githubusercontent.com/60191958/106815793-5c755480-6642-11eb-8fa9-d5e30c9c3155.png)

## TODO

- Switch to tokio single threaded (features = rt-core instead of rt-threaded)
	- Will enable us to use Rc instead of Arc and druid::im-rc instead of druid::im
    - Requires tokio to be running on a separate thread from UI thread (2 threaded application)
- Loop reconnect on disconnect with reconnecting modal
    - On first occurrence, don't prompt user, do it automatically for automatic connecting on first launch
    - kill ux
	- Add restart LCU button to reconnect modal
