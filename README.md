# Reduce Image Size

Reduces size of images in a folder (and optionally sub-folders, recursively).

This is useful for archiving of photos, for example, as they look the same on a display even with a reduced file size.  
This application reduces the file sizes of the images in bulk.

Keeps the original images and creates copies with reduced file size.

Copies the entire folder tree, with all sub-folders that exist in the source tree.  
The target folder tree will be created automatically, and the new reduced-size images will be copied properly to their respective paths.  
It is only required to provide the root target folder, and it will also be created if it doesn't exist.

The destination folder can be the same as the source folder, in which case the original images will be overwritten, and not retained.

Options:
- Look into subdirectories recursively (process the entire tree); recommended: `-r`, `--recursive`
- Reduce both image dimensions by half: `--resize`
- JPEG quality, on a scale from 0 (worst) to 95 (best); the default is 75; ignored in case of PNGs: `--quality <QUALITY>`

Written in Rust 1.74.1.  
Tested on Windows 11 with JPEGs and PNGs.  
Might work with other image formats, too, but this hasn't been tested.  
Other OSes haven't been tested, but should work.

## Running the Application
### Windows
Go to the directory with the executable and run the program run as:  
`reduce_image_size.exe <source_folder> <destination_folder> [options]`

Or, provide full path to the program.

Paths to the source and destination folders can be absolute or relative.
