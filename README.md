# UNICORN
UNIversal COmpression for Reliable Normalization

![UNICORN](./docs/images/icon.webp)


## Description
UNICORN is a tool for handling archive files with just one command. You can handle not only zip files but also tar.gz files and tar.xz files.
This eliminates the need to memorize commands for handling archive files.


## Installation
```sh
$ brew install unicorn
```


## Usage
```sh
$ unicorn [OPTIONS] <ARGUMENTS...>
OPTIONS
    -m, --mode <MODE>       Mode of operation. available: extract, archive, and auto.
                            Default is auto.
    -d, --dest <DEST>       Destination of the extraction results.
                            Default is the current directory.
    -o, --output <FILE>     Output file for the archive.
                            Default is the totebag.zip.
                            The archive formats are guessed form extension of the file name.
        --overwrite         Overwrite the output file if it exists.
    -v, --verbose           Display verbose output.
    -h, --help              Display this help message.

ARGUMENTS
    extract mode:   archive files to be extracted.
    archive mode:   files to be archived.
    auto mode:      if the arguments have archive files, it will extract them.
                    Otherwise, it will archive the files.
```
