# 💼 aegis manager

Convenient [DMP aegis] development.

[DMP aegis]: https://bitbucket.org/mbizcoid/aegis/src/master

## Features

### Manage docker apps

- Start aegis containers.
```sh
aegis start  # Lists available profiles, e.g. Frontend, Backend, Mobile, and All
aegis start 1
# markdown profiles table?
# 1  Frontend: redis nginx cuirass bloodthorn tarrasque
# 11 Alunalun: redis nginx tarrasque
```

- Stop some (interactively) or all aegis containers.
```sh
aegis stop -i  # Opens tui. Select an app to stop.
aegis stop  # Wrapper for `docker stop $(docker ps --filter "name=aegis" -q)`
```

<br>

## Install
```sh
brew tap tbmreza/tools
brew install aegis-manager
```

<br>

## Testing

```sh
cargo test
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
