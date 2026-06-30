# t3s3-epaper

Firmware support library and examples for the [LilyGO T3-S3](https://www.lilygo.cc/products/t3-s3-meshtastic) e-paper board (SX1262 LoRa variant).

The library itself is pure `embedded-hal` 1.0 and stays MCU-agnostic. It provides:

- `board` — GPIO pin assignments and defaults for the T3-S3 e-paper board
- `sx1262` — a blocking driver for the Semtech SX1262 LoRa radio
- `ssd1680` — a driver for the SSD1680 e-paper display (full and partial updates)

All ESP32-S3 platform glue lives in the examples (`dev-dependencies`), so building
the library on a host needs no special toolchain.

## requirements

The examples target `xtensa-esp32s3-none-elf` and need the Espressif Rust toolchain:

- The `esp` Rust toolchain. Install via [`espup`](https://github.com/esp-rs/espup):
  ```bash
  cargo install espup
  espup install
  source ~/export-esp.sh   # adds the esp toolchain + xtensa GCC to your env
  ```
  (the workspace root pins `channel = "esp"` in `rust-toolchain.toml`.)
- [`espflash`](https://github.com/esp-rs/espflash) for flashing and monitoring:
  ```bash
  cargo install espflash
  ```

The target, runner and `build-std` are configured at the workspace root
(`.cargo/config.toml`), and the `linkall.x` linker script is supplied by this
crate's `build.rs`, so the `cargo run --example` commands below just work once the
toolchain is installed and the board is plugged in.

## running the examples

Plug the board in over USB, then flash one of the examples. Each builds, flashes,
and opens a serial monitor:

```bash
cargo run -p t3s3-epaper --example display            # draw text + a partial-update demo on the e-paper
cargo run -p t3s3-epaper --example tx                 # send an incrementing LoRa packet every ~3s
cargo run -p t3s3-epaper --example rx                 # print + display every received packet (with RSSI/SNR)
just ble                                              # BLE ⇄ LoRa bridge (drive with tools/ble.py) — see below
cargo run -p t3s3-epaper --example wifi_lora_bridge   # Wi-Fi ⇄ LoRa bridge with a web UI
```

### ble

The board advertises a Nordic UART Service as `T3S3-Msg` and bridges a BLE central
to the LoRa radio, mirroring both directions to the e-paper. Drive it from a host
with `tools/ble.py` (a `uv` script): `--send` a message, `--listen` for incoming
packets, or `--interact` for a REPL. See the `examples/ble.rs` module docs for the
service UUIDs and the dual-core task placement.

Run it with `just ble` from the workspace root — esp-radio's scheduling is
timing-sensitive, so the recipe sets `ESP_RTOS_CONFIG_TICK_RATE_HZ=1000` (a 1 ms
scheduler tick) so the radio thread can service advertising events on time. The
workspace dev profile already builds `esp-hal`/`esp-radio`/`esp-rtos` at
`opt-level = 3`, so a plain `just ble` (debug) build is enough; the equivalent is
`ESP_RTOS_CONFIG_TICK_RATE_HZ=1000 cargo run -p t3s3-epaper --example ble`.

### wifi_lora_bridge

The board hosts an open Wi-Fi access point (SSID `lora-tx`) plus a tiny web page.
Join the network from a phone and the captive portal should open. Type a message and send
it out over LoRa. Incoming LoRa packets are listed live on the same page and shown
on the e-paper. 
