# Blog Archiver

Blog Archiver is a command-line tool written in Rust that downloads and saves blog posts from an archived website. It fetches the HTML content of specified blog posts, extracts the main content, and saves it as Markdown files for offline access or backup purposes.

DISCLAIMER: this is a one-off project built specifically for a blog salvation effort from [web.archive.org](https://web.archive.org/). It is built entirely based on ChatGPT 4o prompts (including this README), in order to automate the tedious task of copying 150 blog pages and learn a few concepts along the way. It is not intended as a generic application, but could be adapted easily I suppose.  Ensure you have the necessary permissions to download and store content from the target website.

## Features

- Fetch Base HTML: The tool fetches the HTML content of the provided base URL.
- Extract Post Links: It parses the HTML to find links to individual blog posts matching a specific pattern.
- Filter Links: Filters out links that don't match the desired pattern (e.g., URLs without a title or containing fragments).
- Download and Save Posts: For each valid post link, it fetches the HTML content, extracts the title and main content, and saves them as a Markdown file in the specified output directory. Processing is done concurrently to increase efficiency. Displays progress with a progress bar.

## Use
A stand-alone executable for Windows x64 is available as release. No installation required, just put it at a desired location and run it from the command line.

```blog-archiver --base_url <BASE_URL> --output_dir <"OUTPUT_DIR">```

where ```base_url``` is the URL to the blog archive page (that contains links to the individual post pages), and ```output_dir``` is where you want the resulting markdown files to be stored (one per post page, named by post title). Use quotation marks if your output_dir path on Windows contains spaces.

Example:
```blog-archiver --base_url https://web.archive.org/web/20231202013114/http://blog.com/blog/ --output_dir "C:\user\My Documents\downloaded_posts"```

## Adapt and build

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- Clone the repository
- Navigate to the project repository
- Optionally, adapt the code
- Run the project from cargo (```cargo run -- <BASE_URL> <"OUTPUT_DIR">```), or build the project for your system (```cargo build --release```), and run the executable from ```./target/release/blog-archiver```

The project utilizes the following Rust crates:
- clap: For command-line argument parsing.
- futures-util: For asynchronous operations.
- scraper: For HTML parsing and element selection.
- reqwest: For making HTTP requests.
- indicatif: For displaying progress bars.
- regex: For regular expression matching.
- tokio: For asynchronous runtime.

## Acknowledgments

Special thanks to the authors of the Rust crates used in this project for providing excellent tools that made this project possible.

