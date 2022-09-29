# 💼 aegis manager

Convenient [DMP aegis] development.

[dmp aegis]: https://bitbucket.org/mbizcoid/aegis/src/master

## Features

### Troubleshoot developer's local environment

```sh
aegis checkhealth [-y]
```

Common troubles are grouped based on whether or not it makes sense to automate the fix.
The autofixes must be safe to perform even if status quo is already correct.

#### 1. Troubles reported, fixes suggested.

| Trouble                                                                   | Fix                          |
| ------------------------------------------------------------------------- | ---------------------------- |
| `aegis-nginx` container hasn't been restarted since it was first started. | `docker restart aegis-nginx` |
| `vladmir` and `voodoo` are not running on port xx and yy.                 | `npm run dev`                |

#### 2. Troubles reported, fixes suggested, suggestions performed (on user confirmation).

| Trouble                          | Fix                                                                   |
| -------------------------------- | --------------------------------------------------------------------- |
| VPN state is disconnected.       | `/opt/cisco/anyconnect/bin/vpn -s connect vpn-inter.mbizmarket.my.id` |
| No aegis containers are running. | `aegis start`                                                         |

### Manage docker apps

- Start aegis containers.

```sh
aegis start [PROFILE]
```

| Profile name | Applications                             | PROFILE |
| ------------ | ---------------------------------------- | ------- |
| Frontend     | redis nginx cuirass bloodthorn tarrasque | 1       |
| Alunalun     | redis nginx tarrasque                    | 11      |

- Stop all aegis containers. Wrapper for `docker stop $(docker ps --filter "name=aegis" -q)`.

```sh
aegis stop
```

- Manage (start, stop) individual aegis container via interactive list.

```sh
aegis list
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
