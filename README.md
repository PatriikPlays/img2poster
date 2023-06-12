# img2poster

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/PatriikPlays/img2poster?include_prereleases&label=Latest%20release&style=flat-square)

Image to poster CLI converter tool for [SwitchCraft3](https://sc3.io) posters.

img2poster is written in rust for speed and efficency! It is faster than most other implementations of 2dj(a).

## Installation

### Windows

1. Download the latest windows executable from GitHub releases. `img2poster-win.exe`
2. Open your terminal (cmd)
3. Use it! Check out usage guide below.

### Linux

1. Download the latest Linux executable from GitHub releases. `img2poster-lnx`
2. Open your terminal of choice (bash)
3. Use it! Check the usage guide below.

## Usage

img2poster is a CLI to convert images to posters. See the CLI argument and flags below:

### Input **(Required)**

The input argument is the file path to the source image you want to convert into a poster.

Aliases:

* `-i`
* `--input`

Examples:

* `-i apioform.png`
* `-i /home/me/images/capy64.jpg`
* `--input ../mfw.jpg`

### Output **(Required)**

The output argument is the file path where you want your poster files to end up.

> **Note**
> You should not include the file extension. This is automatically set by the program.

Aliases:

* `-o`
* `--output`

Examples:

* `-o myPoster`
* `--output mySuperCoolPoster`

Do **not** do this:

* `-o myPoster.2dj`
* `--output anotherPoster.2dja`

### Scale X

The scale-x argument is the amount of pixels on the X axis to scale the poster to. A single poster is always 128x128, which means that **this field has to be a multiple of 128.**

Aliases:

* `-x`
* `--scalex`

Examples:

* `-x 256`
* `-x 384`
* `--scalex 128`

### Scale Y

The scale-y argument is the amount of pixels on the Y axis to scale the poster to. A single poster is always 128x128, which means that this field has to be a multiple of 128.

Aliases:

* `-y`
* `--scaley`

Examples:

* `-y 256`
* `-y 384`
* `--scaley 128`

### Poster Label

The poster label argument is what to label the poster as.

> **Note**
> The poster label cannot be longer than 25 characters

Aliases:

* `-l`
* `--label`

Examples:

* `-l myPoster`
* `--label ApioformPoster2`

### Force poster label

The force poster label argument is able to overwrite the actual label, instead of the default `<Label>: (x,y)/(totalX*totalY)`.

Aliases:

* `-L`
* `--forcelabel`

Examples:

* `-L FunnyPoster`
* `--forcelabel veryPostery`

### Force poster tooltip

The force poster tooltip argument is similar to the "Force poster label" argument, except this one overwrites the tooltip instead of the default JSON information

Aliases:

* `-T`
* `--forcetooltip`

Examples:

* `-T MyCoolTooltip`
* `--forcetooltip VeryTooltip`

### Per poster quantization

The per poster *quantization* **flag** makes the program select the colorpalette on a per-poster basis.

Aliases:

* `-Q`
* `--perPosterQuantization`

Examples:

* `-Q`
* `--perPosterQuantization`
