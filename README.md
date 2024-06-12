# img2poster

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/PatriikPlays/img2poster?include_prereleases&label=Latest%20release&style=flat-square)

Rust based CLI tool for converting and manipulating [SwitchCraft3](https://sc3.io) posters.

## Installation

### Windows

1. Download the latest windows executable from GitHub releases (`img2poster-win.exe`)
2. Open your terminal (cmd)
3. Use it! Check out usage guide below

### Linux

1. Download the latest Linux executable from GitHub releases (`img2poster-lnx`)
2. Open your terminal of choice
3. Use it! Check the usage guide below

## Usage

### Input

> [!NOTE]
> This argument is required

The input argument is the file path to the source image/poster you want to convert into a poster/image.

Syntax:

- `-i <INPUT_FILE>`
- `--input <INPUT_FILE>`

Examples:

- `-i ./apioform.png`
- `-i /home/me/images/capy64.jpg`
- `--input ~/mfw.jpg`
- `-i ./poster.2dj`
- `-i ./poster.2dja`

---

### Output

> [!NOTE]
> This argument is required

The output argument is the file path where you want your poster/image files to end up.

Syntax:

- `-o <OUTPUT_FILE>`
- `--output <OUTPUT_FILE>`

Examples:

- `-o ./myPoster.2dj`
- `--output ~/anotherPoster.2dja`
- `-o /home/me/myImage.png`
- `--output ./myImage.jpg`

---

### Preview

It is possible to preview the poster in a normal image format.
To do this specify the preview argument with a path.

Syntax:

- `-p <PREVIEW_OUTPUT_FILE>`
- `--preview <PREVIEW_OUTPUT_FILE>`

Examples:

- `-p ~/myPreview.png`
- `-p ../myPreview.bmp`
- `--preview ./myPreview.jpg`
- `--preview /home/me/myPreview.jpeg`

---

### Autoscale

Autoscale automatically scales the image to its original resolution, rounded to nearest 128px.

Aliases:

- `-a <IMAGE_SCALE>`
- `-autoscale <IMAGE_SCALE>`

Examples:

- `--autoscale 2` will resize to 2x the size of the input image
- `-a 1` will resize to the closest possible size of the input image

---

### Scale X

The scale-x argument is the amount of pixels on the X axis to scale the poster to.
A single poster is always 128x128, which means that **this field has to be a multiple of 128.**

> [!NOTE]
> Only use with image input files, not 2dj/2dja

> [!NOTE]
> Cannot be used with autoscale

Aliases:

- `-x`
- `--scale-x`

Examples:

- `-x 256`
- `-x 384`
- `--scale-x 128`

---

### Scale Y

The scale-y argument is the amount of pixels on the Y axis to scale the poster to.
A single poster is always 128x128, which means that this field has to be a multiple of 128.

> [!NOTE]
> Only use with image input files, not 2dj/2dja

> [!NOTE]
> Cannot be used with autoscale

Aliases:

- `-y`
- `--scale-y`

Examples:

- `-y 256`
- `-y 384`
- `--scale-y 128`

---

### Resizing Algorithm

The algorithm to use for scaling the input.

Acceptable values:

- `nearest`
- `triangle`
- `catmull-rom` (default)
- `gaussian`
- `lanczos3`

Syntax:

- `-r <RESIZE_ALGORITHM>`
- `--resize-algorithm <RESIZE_ALGORITHM>`

Examples:

- `-r nearest`
- `--resize-algorithm lanczos3`

---

### Poster Label

The poster label argument is what to label the poster as.

> [!NOTE]
> Only use with image input files, not 2dj/2dja

> [!NOTE]
> The poster label cannot be longer than 25 characters

> [!NOTE]
> The label will end up as `<LABEL>: (x,y)/(totalX*totalY)`. To force your own label use -L (see below)

Syntax:

- `-l <LABEL>`
- `--label <LABEL>`

Examples:

- `-l myPoster`
- `--label ApioformPoster2`

---

### Force poster label

The force poster label argument overwrites the actual label, instead of the default `<LABEL>: (x,y)/(totalX*totalY)`.

> [!NOTE]
> Only use with image input files, not 2dj/2dja

Syntax:

- `-L <FORCED_LABEL>`
- `--forcelabel <FORCED_LABEL>`

Examples:

- `-L FunnyPoster`
- `--forcelabel veryPostery`

---

### Force poster tooltip

The force poster tooltip argument overwrites the default tooltip with the supplied string.

> [!NOTE]
> Only use with image input files, not 2dj/2dja

Snytax:

- `-T <TOOLTIP>`
- `--forcetooltip <TOOLTIP>`

Examples:

- `-T MyCoolTooltip`
- `--forcetooltip VeryTooltip`

---

### Per poster quantization

The per poster quantization flag makes the program select the colorpalette on a per-poster basis.

> [!NOTE]
> Only use with image input files, not 2dj/2dja

Syntax:

- `-Q`
- `--per-poster-quantization`

Examples:

- `-Q`
- `--per-poster-quantization`
