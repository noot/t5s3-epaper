# t5s3-epaper-core

Rust driver library for the [LilyGo T5 E-Paper S3 Pro](https://lilygo.cc/products/t5-e-paper-s3-pro)
device family (ESP32-S3, ED047TC1 4.7" panel): display, touch, RTC, SD, LoRa, GPS,
power, and frontlight, plus all the hardware examples.

Forked from
[azw413/lilygo-t5s3paperpro-rs](https://github.com/azw413/lilygo-t5s3paperpro-rs)
(itself a fork of [fridolin-koch/lilygo-epd47-rs](https://github.com/fridolin-koch/lilygo-epd47-rs)).

Hardware behavior was reverse-engineered from the vendor firmware at
[Xinyuan-LilyGO/T5S3-4.7-e-paper-PRO](https://github.com/Xinyuan-LilyGO/T5S3-4.7-e-paper-PRO).

## examples

Examples live in this crate. Plain examples need no extra features:

```sh
cargo run -p t5s3-epaper-core --example hello-world
```

Some examples require a feature flag (declared via `required-features`, so cargo
will tell you if one is missing):

```sh
cargo run -p t5s3-epaper-core --example gps   --features gps          # GPS module readout
cargo run -p t5s3-epaper-core --example lora  --features lora         # LoRa radio
cargo run -p t5s3-epaper-core --example clock --features wifi         # wifi NTP clock (see the ui readme for .env)
```

| Example | Feature | Description |
| --- | --- | --- |
| `hello-world` | — | Minimal draw plus a bundled BMP image. |
| `simple` | — | Smallest embedded-graphics draw. |
| `counter` | — | Partial-refresh counter. |
| `grayscale` | — | 16-level grayscale rendering. |
| `rotation` | — | Display rotation modes. |
| `input` | — | Read the physical buttons. |
| `touchscreen` | — | Read the capacitive touch panel. |
| `frontlight` | — | Drive the frontlight brightness. |
| `battery` | — | Read battery voltage / level. |
| `temperature` | — | Read the onboard temperature sensor. |
| `rtc-clock` | — | RTC-backed clock (no network). |
| `sdcard` | — | List and read files from the microSD slot. |
| `deepsleep` | — | Deep sleep and wake. |
| `screen-repair` | — | Panel repair routine (adapted from LilyGo). |
| `clock` | `wifi` | NTP clock over wifi. |
| `gps` | `gps` | GPS module detection and fix readout. |
| `lora` | `lora` | LoRa send/receive. |
| `lora_send` | `lora` | LoRa transmitter. |
| `lora_recv` | `lora` | LoRa receiver. |

Or via `just` (from the workspace root):

```sh
just clock   # the wifi clock example
just check   # compile-check everything
just lint    # fmt + clippy across the workspace
```
