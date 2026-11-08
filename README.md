# img_utils

Rust helper functions for image editing tasks.

## list_image_paths

Returns a list of image paths in a folder filtered by allowed file extensions.

### Basic Usage

By default, list_image_paths finds all supported image formats (jpg, jpeg, tif, tiff, png, nef, cr2, dng).

```rust
use img_utils::list_image_paths;

let image_paths: Vec<PathBuf> = list_image_paths("path/to/image/folder");

```
