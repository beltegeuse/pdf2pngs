# PDF to PNGs

Simple tool based on `poppler`, `cairo` and `glib` to transform PDF into multiple PNGs files. Only tested on Linux.

```
PDF2PNGs 1.0
beltegeuse <adrien.gruson@gmail.com>
Convert PDF to multiple PNG images

USAGE:
    pdf2pngs [OPTIONS] <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p <max_pages>        Sets the maximum pages to export [default: 0]
    -s <scale>            Sets image scale [default: 1.0]

ARGS:
    <input>     PDF input filepath
    <output>    Output basename
```

Original code: https://codereview.stackexchange.com/questions/234028/creating-a-thumbnail-of-a-pdf-using-cairo-poppler-and-rust

Added command line, image scale and max_pages support.