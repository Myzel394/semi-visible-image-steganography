## Experiments

### Environment

To test out how well the information is preserved when using JPEG compression, I compressed the images with different on / off colors differences and JPEG qualities.

Here are all commands used:

* Generate image: `image-pixel-info --image-width 1920 --image-height 1080 "created by myzel394" --on-color "$1" --off-color "$2" --fill-color "#000"`
* Compress image: `magick image.png -quality $3 out.jpg`

$1 = on color
$2 = off color

* "1 value difference" = there's one number difference between the colors, like: "#030914" and "#030814"
* "1 lightness difference" = there's one lightness difference between the colors. I used HSL to add / remove one lightness value, like: "#030916" and "#040B1B" 


### Results

The following results all used 8x8 blocks.

* Using a 1 value difference with `quality 90` resulted in **preserving the information**.
* Using a 1 value difference with `quality 80` resulted in **losing the information**.
* Using a 1 lightness difference with `quality 50` resulted in **preserving the information** - However, the blobs were clearly visible to me.


* Using a 14 value difference with `quality 90` and 1x1 blocks resulted in **losing the information**.
* Using a 14 value difference with `quality 90` and 10x10 blocks resulted in **preserving the information** - Plus, the blobs look funny.


Try to use PNG instead of JPEG if you want to preserve the information.
