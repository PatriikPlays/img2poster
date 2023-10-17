# img2poster

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/PatriikPlays/img2poster?include_prereleases&label=Latest%20release&style=flat-square)

Image to poster CLI converter tool for [SwitchCraft3](https://sc3.io) posters.

img2poster is written in rust for speed and efficiency! It is faster than most other implementations of 2dj(a).

## Installation

### Windows

1. Download the latest windows executable from GitHub releases. `img2poster-win.exe`
2. Open your terminal (cmd)
3. Use it! Check out usage guide below.

### Linux

1. Download the latest Linux executable from GitHub releases. `img2poster-lnx`
2. Open your terminal of choice
3. Use it! Check the usage guide below.

## Usage (for 0.0.9)

img2poster is a CLI to convert between images and posters. See the CLI argument and flags below:

---
### Input **(Required)**

The input argument is the file path to the source image/poster you want to convert into a poster/image.

Aliases:

* `-i`
* `--input`

Examples:

* `-i ./apioform.png`
* `-i /home/me/images/capy64.jpg`
* `--input ../mfw.jpg`
* `-i ./poster.2dj`
* `-i ./poster.2dja`

---
### Output **(Required)**

The output argument is the file path where you want your poster/image files to end up.

Aliases:

* `-o`
* `--output`

Examples:

* `-o ./myPoster.2dj`
* `--output ./anotherPoster.2dja`
* `-o ./myImage.png`
* `--output ./myImage.jpg`

---
### Preview

The preview argument is the file path where you want your preview image to end up.

Aliases:
* `-p`
* `--preview`

Examples: 

* `-p ./myPreview.png`
* `--preview ./myPreview.jpg`
* `-p ./myPreview.bmp`
* `--preview ./myPreview.jpeg`

---
### Scale X

The scale-x argument is the amount of pixels on the X axis to scale the poster to. A single poster is always 128x128, which means that **this field has to be a multiple of 128.**

> Note
> Only use with image input files, not 2dj/2dja

Aliases:

* `-x`
* `--scale-x`

Examples:

* `-x 256`
* `-x 384`
* `--scale-x 128`

---
### Scale Y

The scale-y argument is the amount of pixels on the Y axis to scale the poster to. A single poster is always 128x128, which means that this field has to be a multiple of 128.

> Note
> Only use with image input files, not 2dj/2dja

Aliases:

* `-y`
* `--scale-y`

Examples:

* `-y 256`
* `-y 384`
* `--scale-y 128`

---
### Poster Label

The poster label argument is what to label the poster as.

> Note
> Only use with image input files, not 2dj/2dja

> **Note**
> The poster label cannot be longer than 25 characters

Aliases:

* `-l`
* `--label`

Examples:

* `-l myPoster`
* `--label ApioformPoster2`

---
### Force poster label

The force poster label argument is able to overwrite the actual label, instead of the default `<Label>: (x,y)/(totalX*totalY)`.

> Note
> Only use with image input files, not 2dj/2dja

Aliases:

* `-L`
* `--forcelabel`

Examples:

* `-L FunnyPoster`
* `--forcelabel veryPostery`

---
### Force poster tooltip

The force poster tooltip argument is similar to the "Force poster label" argument, except this one overwrites the tooltip instead of the default JSON information

> Note
> Only use with image input files, not 2dj/2dja

Aliases:

* `-T`
* `--forcetooltip`

Examples:

* `-T MyCoolTooltip`
* `--forcetooltip VeryTooltip`

---
### Per poster quantization

The per poster *quantization* **flag** makes the program select the colorpalette on a per-poster basis.

> Note
> Only use with image input files, not 2dj/2dja

Aliases:

* `-Q`
* `--per-poster-quantization`

Examples:

* `-Q`
* `--per-poster-quantization`
