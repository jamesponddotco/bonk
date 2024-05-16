# `bonk`

`bonk` is a very simple, yet powerful, command-line application that
uses machine learning to detect whether or not an image contains nudity.

The tool analyzes image files or directories of images and provides
predictions on the likelihood of each image containing nudity.

**Features**

- Detects nudity in images using a pre-trained machine learning model.
- Supports various image formats including JPEG, PNG, GIF, and WebP.
- Can process individual image files or entire directories of images.
- Allows customization of the probability threshold for nudity detection.
- Enables parallel processing of images for improved performance.
- Generates all output in JSON format for easy parsing and processing.

The name `bonk` comes from the ["Go to horny jail"
meme](https://knowyourmeme.com/memes/go-to-horny-jail), because why not?

**Why?**

`bonk` exists for two reasons:

1. I needed a way to help automate at least part of the review process
   for customer websites. The company I rent servers from don't allow
   nudity, so I needed a way to detect it.
2. I wanted to learn Rust.

> This is my first time writing Rust and I haven't even finished the
> Rust book yet. Keep that in mind when reviewing the code.

## Installation

### From source

First, ensure the following dependencies are installed:

- Rust.
- Cargo.
- make.
- [scdoc](https://git.sr.ht/~sircmpwn/scdoc).

Then clone the repository, compile, and install:

```bash
make
sudo make install
```

## Usage

```bash
$ bonk --help
Use machine learning to detect nudity in images.

Usage: bonk [OPTIONS] <PATH>

Arguments:
  <PATH>  path to an image file or directory of images to analyze

Options:
  -t, --threshold <THRESHOLD>  probability threshold above which an image is...
  -p, --parallel <PARALLEL>    number of images to process concurrently [default: 16]
  -h, --help                   Print help
  -V, --version                Print version
```

Refer to the _bonk(1)_ manpage after installation for examples and more
usage information.

## Output

The tool outputs the analysis results in JSON format. For each image, it
provides the following information:

- **has_nudity**: A boolean indicating whether the image is considered
  to contain nudity based on the specified threshold.
- **path**: The path to the image file.
- **filename**: The name of the image file.
- **predictions**: An array of prediction objects, each containing a
  category and its corresponding probability.

**Example output**

```json
{
  "has_nudity": true,
  "path": "/path/to/image",
  "filename": "example.jpg",
  "predictions": [
    {
      "category": "drawing",
      "probability": 0.00024631553
    },
    {
      "category": "hentai",
      "probability": 0.0049908394
    },
    {
      "category": "neutral",
      "probability": 0.0015131987
    },
    {
      "category": "porn",
      "probability": 0.9691646
    },
    {
      "category": "sexy",
      "probability": 0.024084995
    }
  ]
}
```

## Contributing

Anyone can help make `bonk` better. Send patches on the [mailing
list](https://lists.sr.ht/~jamesponddotco/bonk-devel) and report bugs on
the [issue tracker](https://todo.sr.ht/~jamesponddotco/bonk).

You must sign-off your work using `git commit --signoff`. Follow the
[Linux kernel developer's certificate of
origin](https://www.kernel.org/doc/html/latest/process/submitting-patches.html#sign-your-work-the-developer-s-certificate-of-origin)
for more details.

All contributions are made under [the GPL-2.0 license](LICENSE.md).

## Acknowledgements

- The code in this project draws inspiration from by the Rust library
  [Fyko/nsfw](https://github.com/Fyko/nsfw) and the JavaScript library
  [infinitered/nsfwjs](https://github.com/infinitered/nsfwjs).
- The machine learning model comes from
  [GantMan/nsfw_model](https://github.com/GantMan/nsfw_model) and was
  just converted to ONNX. Copyright belongs to the authors.

## Resources

The following resources are available:

- [Support and general discussions](https://lists.sr.ht/~jamesponddotco/bonk-discuss).
- [Patches and development related questions](https://lists.sr.ht/~jamesponddotco/bonk-devel).
- [Instructions on how to prepare patches](https://git-send-email.io/).
- [Feature requests and bug reports](https://todo.sr.ht/~jamesponddotco/bonk).

---

Released under the [GPL-2.0 license](LICENSE.md).
