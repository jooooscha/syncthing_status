# Status information about local or remote Syncthing instances

- Place your config in `~/.config/syncthing_status/devices.yml`
- Your can retrieve your `api_key` from the web panel of your Syncthing instance
- This is designed to be used in your status bar (Polybar, etc.)

#### Example config

```
---
- url: "https://localhost:8384"
  short_name: "M"
  name: "main station"
  api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

- url: "https://192.168.0.20:8384"
  short_name: "R"
  name: "Raspberry Pi"
  api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
```
