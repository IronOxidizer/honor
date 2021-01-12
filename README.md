# Honor
A fast and stable League of Legends client built with Rust 

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

Honor isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

Honor was created under Riot Games' "Legal Jibber Jabber" policy using assets owned by Riot Games.  Riot Games does not endorse or sponsor this project.

## TODO

- Switch to tokio single threaded (features = rt-core instead of rt-threaded)
	- Will enable us to use Rc instead of Arc and druid::im-rc instead of druid::im
- Loop reconnect on disconnect with reconnecting modal
    - On first occurence, don't prompt user, do it automatically for automatic connecting on first launch
    - kill ux
	- Add restart LCU button to reconnect modal