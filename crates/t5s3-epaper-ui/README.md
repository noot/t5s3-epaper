# t5s3-epaper-ui

Touchscreen UI firmware for the [LilyGo T5 E-Paper S3 Pro](https://lilygo.cc/products/t5-e-paper-s3-pro):
a wifi NTP clock, a LoRa keyboard messenger, GPS, SD-card wallpapers, and Music /
Environment pages that pull live data from [`noot-server`](https://github.com/noot/noot-server).
Built on [`t5s3-epaper-core`](../t5s3-epaper-core).

## flashing

The UI bakes wifi credentials in at build time, so configure them first (from the
workspace root):

```sh
cp .env.example .env    # then fill in SSID / PASSWORD / TZ_OFFSET_HOURS
```

The Music and Environment pages fetch JSON from `noot-server` over wifi, so also
set `SERVER_HOST` / `SERVER_PORT` (the server address) and `SENSOR_ID` (the
sensor device the Environment page reads) in `.env`.

Then flash and monitor:

```sh
just ui
# equivalent to:
SSID=… PASSWORD=… TZ_OFFSET_HOURS=… SERVER_HOST=… SERVER_PORT=… SENSOR_ID=… \
  cargo run -p t5s3-epaper-ui --features gps
```

notes:

- GPS support is optional — drop `--features gps` to build the UI without it (the
  GPS page then shows a "compile with --features gps" hint).
- The Music and Environment pages bring wifi up on entry to fetch from
  `noot-server`, then power the radio back down (tap anywhere to refresh). With
  the server unreachable they show an error line rather than blocking.
- The Music page shows album art (decoded on-device from the cover the server
  serves) and has transport + volume buttons. A button press keeps the page up
  and shows "contacting server..." then "ok"/"error" (auto-dismissed after a few
  seconds) on a status line at the bottom; it still takes a few seconds for the
  wifi round-trip. Play/pause and volume apply with a partial refresh (no
  full-screen flush); next/previous do a full refresh so the new album art
  re-renders (the grayscale art can't be partial-refreshed cleanly). Note the
  server only forwards controls to backends that support them (Spotify); for a
  Navidrome-only setup the buttons are no-ops.
- The song position updates live without wifi: it's extrapolated from the device
  clock and the progress line is repainted every few seconds. The radio only
  comes back when the track ends (to pick up the next one), on a control tap, or
  on re-entering the page.
- The UI loads wallpapers as BMP files from `WALLS/` in the SD card root; use
  `tools/wallpaper` to prepare them.
