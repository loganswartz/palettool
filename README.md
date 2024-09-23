# palettool

A simple tool to build a palette from regions of a photo.

## About

I created this because I wanted an easy way to a create a palette based on the
most unique colors of a detailed photo or screenshot. I find that oftentimes,
checking a single pixel in the middle of a photo ends up not matching what I
intuitively expect the color to be. The solution to this is to average the
color of many pixels in a region, which this program does automatically.

We use Oklab when averaging out the colors of each region, since it has
excellent perceptual uniformity.

## Usage

To build:

```bash
cargo build
```

Then, call `palettool` with a single file and a list of rectangle coordinates. A
rectangle is specified with the format `x1,y1-x2,y2`, where `x1,y1` is the top
left corner, and `x2,y2` is the bottom right corner.

For example, if I want to get the average color of the regions `0,0` to
`100,100` and `400,300` to `500,500` in `photo.jpg`:

```bash
palettool photo.jpg 0,0-100,100 400,300-500,500
```
