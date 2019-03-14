# About
A quick and handy image converter developed at work for a project. Not anymore in development.

It supports a few image types (TIFF, GIF, JPEG, PNG) and converts them in JPEGs using predefined settings.

Uploaded for newcomers of Rust (like me!) as an example of Rust + gtk-rs, as there still aren't many repositories around.


# Usage
To build and run the code just use:

```
sh scripts/run.sh
```

and it will check the GTK dependency and start building.

The only way it's meant to work is by placing the images in a nested structure like this:

:file_folder: Main folder
 - :file_folder: A
   - :page_facing_up: image.tiff
   - :page_facing_up: image.jpeg
   - :page_facing_up: image.gif

 - :file_folder: B
   - :page_facing_up: image.jpg
   - :page_facing_up: image.png

- :file_folder: C
   - :page_facing_up: image.jpg
   - :page_facing_up: image.png

Using a single folder to contain a multiple set of folders containing the images you want to convert, no matter what the extensions of the files is.


# Screenshots

The main window:

![Imgur](https://i.imgur.com/6CCcGWu.png)

Settings window:

![Imgur](https://i.imgur.com/wRZPzwQ.png)
