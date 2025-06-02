from PIL import Image
import rounded

def run(source):
    sizes = [32, 128, 256, 512]
    png_paths = [f"{size}x{size}.png" for size in sizes]
    ico_path, icns_path, sizes_icon = "icon.ico", "icon.icns", [16, 32, 64, 128, 256, 512, 1024]

    try:
        with Image.open(source) as img:
            # TODO: Preparing to generate the images.
            img = img.convert("RGBA")

            # TODO: Generate the files: "32x32.png", "128x128.png", "256x256.png", and "512x512.png".
            for size, output_path in zip(sizes, png_paths):
                resized_img = rounded.apply(img.resize((size, size), Image.Resampling.LANCZOS), r=size // 4)
                resized_img.save(output_path)
                
            # TODO: Generate the "icon.ico" file.
            icon_images = [rounded.apply(img.resize((size, size), Image.Resampling.LANCZOS), r=size // 4) for size in sizes_icon]
            icon_images[-1].save(ico_path, format="ICO", sizes=[(size, size) for size in sizes_icon])

            # TODO: Generate the "icon.icns" file.
            icns_images = [rounded.apply(img.resize((size, size), Image.Resampling.LANCZOS), r=size // 4) for size in sizes_icon]
            icns_images[-1].save(icns_path, format="ICNS", sizes=[(size, size) for size in sizes_icon])

            # Done and done!
            print("Done and done!")
    except Exception as e:
        print(f"Error: {e}")

run("source.jpg")