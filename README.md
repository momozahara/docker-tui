# DOCKER-TUI
A tui that will execute docker compose command on server via [OpenSSH](https://man.openbsd.org/ssh.1).

### Requirements
[OpenSSH](https://man.openbsd.org/ssh.1) \
Linux user with root privilege (if you have to run sudo in order run docker)

## Installation
### Remote
```bash
cargo install --git https://github.com/momozahara/docker-tui.git
```
### Local
```bash
cargo install --path .
```

## Example
<img src="https://github.com/momozahara/docker-tui/blob/main/assets/example.gif?raw=true">