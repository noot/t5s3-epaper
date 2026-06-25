# wallpaper-tool

Converts an image into a **540×960, 16-level grayscale, Floyd–Steinberg-dithered**
24-bit BMP for the LilyGo T5 S3 Paper Pro. Copy the result to the SD card root as
`WALL1.BMP` and it shows as the deep-sleep screensaver (see `examples/ui.rs`,
`show_wallpaper`).

The dithering is the 1-bit algorithm from the `waveshare-epaper` server's
`render.rs`, generalised from 2 levels to the panel's 16 gray levels (so it
keeps gradients instead of collapsing to pure black/white).

## Usage

```sh
tools/wallpaper/convert.sh <input-image> <output.bmp> [WxH]
# e.g.
tools/wallpaper/convert.sh ~/Pictures/photo.jpg WALL1.BMP
```

- Input may be JPEG or PNG; it is center-cropped to fill the target size.
- Default size is `540x960` (the panel in portrait). Pass e.g. `960x540` for landscape.
- Paths resolve against your current directory.
