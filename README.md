# petit-filou

petit-filou or pf is a command-line tool designed for retrieving mp4 videos from wordpress websites. petit-filou features a range of search options, allowing users to finely adjust their search criteria.

[![Continuous Integration](https://github.com/petit-chat/petit-filou/actions/workflows/ci.yaml/badge.svg)](https://github.com/petit-chat/petit-filou/actions)
[![Coverage Status](https://coveralls.io/repos/github/petit-chat/petit-filou/badge.svg?branch=main)](https://coveralls.io/github/petit-chat/petit-filou?branch=main)

## Installation

As petit-filou is written in Rust, you'll need to install
[Rust](https://www.rust-lang.org/) in order to compile it. Then run:

```console
$ git clone https://github.com/petit-chat/petit-filou
$ cd petit-filou
$ cargo build --release
$ mv ./target/release/pf /usr/local/bin
$ pf --version
1.0.0
```

## Usage

```console
$ pf --help
petit-filou searches for mp4 videos for a given wordpress website.

Usage: pf [OPTIONS] <URL> <MODE>

Arguments:
  <URL>
          Wordpress website base URL (e.g. http://example.com)

  <MODE>
          Searching mode

          Possible values:
          - fast: Fetch from posts only
          - slow: Fetch from both posts and media

Options:
      --before <BEFORE>
          Result set published before a given date (cf. https://core.trac.wordpress.org/ticket/41032)

      --modified-before <MODIFIED_BEFORE>
          Result set modified before a given date (cf. https://core.trac.wordpress.org/ticket/41032)

      --after <AFTER>
          Result set published after a given date (cf. https://core.trac.wordpress.org/ticket/41032)

      --modified-after <MODIFIED_AFTER>
          Result set modified after a given date (cf. https://core.trac.wordpress.org/ticket/41032)

  -e, --exclude <EXCLUDE>
          Ensures result set excludes specific IDs

      --categories-exclude <CATEGORIES_EXCLUDE>
          Ensures result set excludes specific category IDs

      --tags-exclude <TAGS_EXCLUDE>
          Ensures result set excludes to specific tag IDs

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Examples

### Retrieve a Maximum of Video URLs

```console
$ pf http://www.example.com slow
```

### Retrieve Video URLs After a Specified Date

```console
$ pf http://www.example.com fast --after 2024-04-06T18:44:41
```

### Retrieve Video URLs Excluding Specific Tags

```console
$ pf http://www.example.com fast --tags-exclude 1 --tags-exclude 2 --tags-exclude 3
```
