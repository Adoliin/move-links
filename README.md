# Move Links
CLI utility to move (or rename) your files to a new location and
redirect all of its symbolic links, to the new path (or name).

## Usage
> executing `mvl --help` to show the help message

```
Move Links <version>
Ben Mefteh F. <benmeft0@gmail.com>
Move (or rename) SOURCE to DEST, and redirect all ot its symbolic links inside of LINKS_DIRECTORY

USAGE:
    mvl <SOURCE> <DEST> --links-dir <LINKS_DIRECTORY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --links-dir <LINKS_DIRECTORY>    Directory in which to search for symbolic links of SOURCE

ARGS:
    <SOURCE>    Source file or directory
    <DEST>      Destination (output) file or directory
```

## How does it work
Move Links works under the hood by calling three commands:
- `find` for finding the SOURCE symbolic links
- `ln` for redirecting the existing symbolic links to DEST
- And finally `mv` for moving SOURCE to DEST

## Installation
Using cargo:
`cargo install move-links`
