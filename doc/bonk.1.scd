bonk(1)

# NAME

bonk - use machine learning to detect nudity in images

# SYNOPSIS

*bonk* [*OPTIONS*] <*PATH*>++
*bonk* [*-h*|*--help*] [*-v*|*--version*]

# DESCRIPTION

*bonk* is a command-line tool that uses a machine learning model to detect
nudity in images. It can analyze a single image file or a directory containing
multiple images.

The tool classifies each image into one of five categories: Drawing, Hentai,
Neutral, Porn, or Sexy. It outputs the classification results in JSON format,
indicating the probability score for each category and whether the image
contains nudity based on a configurable threshold.

# OPTIONS

*-t*, *--threshold* <NUMBER>
	Set the probability threshold above which an image is considered to contain
	nudity. The threshold value should be between 0.0 and 1.0. The default value
	is 0.7.

*-p*, *--parallel* <NUMBER>
	Set the number of images to process concurrently. The default value is the
	number of CPU cores available on the system.

*-h*, *--help*
	Display help information and exit.

*-v*, *--version*
	Display version information and exit.

# ARGUMENTS

<*PATH*>
	The path to an image file or a directory containing image files to analyze.

# RETURN VALUES

*0*
	Success.

*2*
	Directory or image not found.

*5*
	I/O error when reading a directory or image.

*13*
	Permission denied when reading a directory or image.

*22*
	Error when serializing JSON.

*79*
	Error loading the nudity detection model.

# OUTPUT

*bonk* outputs the classification results in JSON format. For a single image,
the output includes:

- *has_nudity*: A boolean indicating whether the image contains nudity based on
the probability threshold.
- *path*: The path to the parent directory of the image file.
- *filename*: The name of the image file.
- *predictions*: An array of prediction objects, each containing:
	- *category*: The category of the prediction (Drawing, Hentai, Neutral,
	Porn, or Sexy).
	- *probability*: The probability score for the category.

For a directory of images, the output is an object containing:

- *images*: An array of image classification results, each following the same
format as the single image output.

# EXAMPLES

*Example 1. Analyze a single image file:*
	$ bonk path/to/image.jpg

*Example 2. Analyze all images in a directory:*
	$ bonk path/to/directory

*Example 3. Set a custom probability threshold:*
	$ bonk -t 0.5 path/to/image.jpg

*Example 4. Process images with a specific level of parallelism:*
	$ bonk -p 4 path/to/directory

# AUTHORS

Maintained by James Pond <james@cipher.host>.
