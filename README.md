# Source Code Scanner

## Features

- ☑️ Scan a project folder
- ☑️ Search for files with a specific extension(s)
- ☑️ Count the total number of files by extension
- ☑️ Count the total sum of lines of code (ignoring empty lines)
- ☑️ Calculate the average, median, and standard deviation of the lines of code
- ☑️ Ignore specified folder(s)
- ☑️ Generate a tiny markdown report
- ☑️ Colorized output
- ☑️ Written in Rust

<p align="center">
  <img width="842" alt="Screenshot" src="https://github.com/RustamIrzaev/source-code-scanner/assets/352703/685ba6e5-51b2-46aa-8647-17e8294b4f7c">
</p>

## Usage

### Prerequisites

- Rust. Ensure that Rust is installed on your system. You can install Rust from [rust-lang.org](https://rust-lang.org).

### Building the project

Clone the repository and build the project:

```bash
git clone https://github.com/RustamIrzaev/source-code-scanner.git
cd source-code-scanner
cargo build --release # builds the project

cd target/release # switch to where the executable is placed
./source-code-scanner -f <FOLDER> -e <EXT> # run! (see options below)
```

### Run the application

Simple run:
`./source-code-scanner -f <folder_path> -e <file_extension>`

With ignored folders:
`./source-code-scanner -f <folder_path> -e <file_extension> -i <folder1>,<folder2>`

With hardcoded ignored folders:
`./source-code-scanner -f <folder_path> -e <file_extension> --hce`

Real example (with report generation and an extended statistics):
`./source-code-scanner -f /home/user/projects/my_project -e rs -i target,.git -r -s`

## Parameters

### General

- `-f`/`--folder`
  - the path to a project folder
  - usage `-f <folder>`
- `-e`/`--extensions`
  - the file extension of the files to be scanned (comma separated)
  - usage `-e <extension1>,<extension2>`
- `-i`/`--ignore`
  - the list of folders to be ignored, comma separated
  - usage `-i <folder1>,<folder2>,<folder3>`
- `-r`/`--report`: generate a small markdown report (\*.md extension)
- `-s`/`--stats`: show additional summary statistics (average, median, std deviation)

### Misc

- `--hce`: use hardcoded list of ignored folders _(see below)_. Please note that `-i` parameter has higher priority than this one
- `--max-depth`
  - the maximum depth of the folder scanning _(default: 10)_
  - usage `--max-depth <depth>`
- `-V`/`--version`: prints the version of the program
- `-h`/`--help`: prints the help message

## Report

The report is generated in the project folder with the name `report_scs.md`.
The demo report is available [here](demo_report.md).

## Ignored folders

At this moment, the list of ignored folders are hardcoded:
`.idea`, `.git`, `node_modules`, `obj`, `bin`, `build`, `out`, `dist`.

However, you can change this directly from the code - see `hardcoded.rs` file for a `EXCLUDE_FOLDERS_EMBEDDED` slice.

## License

The source code is available under the MIT license. See the [LICENSE](LICENSE) file for more info.
