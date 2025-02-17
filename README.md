# petit-filou

petit-filou or pf is a command-line tool designed for retrieving videos from wordpress websites. petit-filou features a range of search options, allowing users to finely adjust their search criteria.

[![ci](https://github.com/petit-chat/petit-filou/actions/workflows/ci.yaml/badge.svg)](https://github.com/petit-chat/petit-filou/actions)
[![codecov](https://codecov.io/gh/petit-chat/petit-filou/graph/badge.svg?token=DXSZBI5DAE)](https://codecov.io/gh/petit-chat/petit-filou)
[![crates.io version](https://img.shields.io/crates/v/pf_cmd?label=pf_cmd)](https://crates.io/crates/pf_cmd)
[![crates.io version](https://img.shields.io/crates/v/pf_lib?label=pf_lib)](https://crates.io/crates/pf_lib)

## Installation

### Binary

Download [latest release](https://github.com/petit-chat/petit-filou/releases/latest).

### Docker

```console
$ docker pull ghcr.io/petit-chat/pf:latest
```

### Build Locally

```console
$ make build
```

## Usage

```console
$ pf --help
Scans WordPress websites to find videos.

Supported MIME types: video/mp4 and video/quicktime (.mov).

Usage: pf [OPTIONS] <URL>

Arguments:
  <URL>
          WordPress base URL (e.g. <http://example.com>)

Options:
      --before <BEFORE>
          Result set published before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>)

      --modified-before <MODIFIED_BEFORE>
          Result set modified before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>)

      --after <AFTER>
          Result set published after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>)

      --modified-after <MODIFIED_AFTER>
          Result set modified after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>)

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
