# rss2x

A command-line tool to convert RSS or HTML input from stdin into readable text

## Features
- Reads RSS or HTML from stdin
- Outputs readable text
- Detects HTML and converts to text using html2text

## Usage

```sh
cat feed.xml | rss2x --output-format text
cat page.html | rss2x
```

### Output formats
- `text` (default): Human-readable text

## License
BSD 3-Clause License

## Author
See LICENSE for copyright and contact
