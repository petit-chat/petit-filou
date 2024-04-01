# petit-filou

petit-filou is a command-line tool designed for retrieving mp4 videos from wordpress websites. petit-filou features a range of search options, allowing users to finely adjust their search criteria.

[![Build status](https://github.com/petit-chat/petit-filou/workflows/ci/badge.svg)](https://github.com/petit-chat/petit-filou/actions)

## Installation

As petit-filou is written in Rust, you'll need to install
[Rust](https://www.rust-lang.org/) in order to compile it. Then run:

```console
$ git clone https://github.com/petit-chat/petit-filou
$ cd petit-filou
$ cargo build --release
$ mv ./target/release/petit-filou /usr/local/bin
$ petit-filou --version
0.0.1
```

## Usage

### Arguments

* `<URL>`: Wordpress website base URL. This should be a valid URL starting with `http` or `https` and containing only the base URL (e.g. `https://domain.tld`).
* `<MODE>`: Searching mode.
  * `fast`: Retrieve video URLs from [posts](https://developer.wordpress.org/rest-api/reference/posts/#list-posts) only.
  * `slow`: Retrieve video URLs from both [posts](https://developer.wordpress.org/rest-api/reference/posts/#list-posts) and [media](https://developer.wordpress.org/rest-api/reference/media/#list-media).

### Options

* `--before <BEFORE>`: Result set published before a given ISO8601 compliant date.
* `--modified-before <MODIFIED_BEFORE>`: Result set modified before a given ISO8601 compliant date.
* `--after <AFTER>`: Result set published after a given ISO8601 compliant date.
* `--modified-after <MODIFIED_AFTER>`: Result set modified after a given ISO8601 compliant date.
* ` -e, --exclude <EXCLUDE>`: Ensures result set excludes specific IDs.
* `--categories-exclude <CATEGORIES_EXCLUDE>`: Ensures result set excludes to specific categorie IDs.
* `--tags-exclude <TAGS_EXCLUDE>`: Ensures result set excludes to specific tag IDs.
* `-h, --help`: Print help.
* `-V, --version`: Print version.

## Examples

### Retrieve a Maximum of Video URLs

```console
$ petit-filou https://www.domain.tld slow
```

### Retrieve Video URLs After a Specified Date

```console
$ petit-filou https://www.domain.tld fast --after 2024-04-06T18:44:41Z
```

### Retrieve Video URLs Excluding Specific Tags

```console
$ petit-filou https://www.domain.tld fast --tags-exclude 1 --tags-exclude 2 --tags-exclude 3
```

## Contributing

See [CONTRIBUTING](https://github.com/petit-chat/petit-filou/CONTRIBUTING.md).

## License

See [LICENSE](https://github.com/petit-chat/petit-filou/LICENSE).
