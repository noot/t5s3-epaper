# t5s3-epaper-ui

Touchscreen UI firmware for the [LilyGo T5 E-Paper S3 Pro](https://lilygo.cc/products/t5-e-paper-s3-pro):
a wifi NTP clock, a LoRa keyboard messenger, GPS, and SD-card wallpapers. Built on
[`t5s3-epaper-core`](../t5s3-epaper-core).

## flashing

The UI bakes wifi credentials in at build time, so configure them first (from the
workspace root):

```sh
cp .env.example .env    # then fill in SSID / PASSWORD / TZ_OFFSET_HOURS
```

Then flash and monitor:

```sh
just ui
# equivalent to:
SSID=… PASSWORD=… TZ_OFFSET_HOURS=… \
  cargo run -p t5s3-epaper-ui --features gps
```

notes:

- GPS support is optional — drop `--features gps` to build the UI without it (the
  GPS page then shows a "compile with --features gps" hint).
- The UI loads wallpapers as BMP files from `WALLS/` in the SD card root; use
  `tools/wallpaper` to prepare them.
