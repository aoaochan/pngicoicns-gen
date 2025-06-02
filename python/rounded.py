from PIL import Image, ImageDraw

def apply(img, r):
    """
    Add rounded corners to an image with anti-aliasing.
    :param img: Input PIL Image object.
    :param r: Radius of the rounded corners.
    :return: Image with rounded corners.
    """
    # TODO: Scale up the mask to apply anti-aliasing
    scale_factor = 4
    big_size = (img.size[0] * scale_factor, img.size[1] * scale_factor)
    mask = Image.new("L", big_size, 0)
    draw = ImageDraw.Draw(mask)
    draw.rounded_rectangle((0, 0) + big_size, radius=r * scale_factor, fill=255)

    # TODO: Downscale the mask to original size for anti-aliasing
    mask = mask.resize(img.size, Image.Resampling.LANCZOS)

    # TODO: Apply the mask to the image
    rounded = Image.new("RGBA", img.size)
    rounded.paste(img, mask=mask)

    return rounded