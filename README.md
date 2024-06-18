# Source Code Scanner

## Features
- ☑️ Scan a project folder
- ☑️ Count the total number of files by extension
- ☑️ Count the total sum of lines of code (ignoring empty lines)
- ☑️ Ignore specified folders
- ☑️ Generate a small markdown report
- ☑️ Colorized output
- ☑️ Written in Rust

<p align="center">
  <img width="842" alt="Screenshot 2024-06-18 at 21 15 55" src="https://github.com/RustamIrzaev/source-code-scanner/assets/352703/685ba6e5-51b2-46aa-8647-17e8294b4f7c">
</p>


## Usage

Simple run:
`cargo run -- -f <folder_path> -e <file_extension>`

With ignored folders:
`cargo run -- -f <folder_path> -e <file_extension> -i <folder1>,<folder2>`

With hardcoded ignored folders:
`cargo run -- -f <folder_path> -e <file_extension> --hce`

Real example (with report generation):
`cargo run -- -f /home/user/projects/my_project -e rs -i target,.git -f`

> Note. If you run the application not with `cargo run` (for example, you built binaries), you should remove `--` from the arguments.

## Parameters
### General
- `-f`/`--folder`
  - the path to a project folder 
  - usage `-f <folder>`
- `-e`/`--extension`
  - the file extension of the files to be scanned
  - usage `-e <extension>`
- `-i`/`--ignore`
  - the list of folders to be ignored, comma separated
  - usage `-i <folder1>,<folder2>,<folder3>`
- `-r`/`--report`: generate a small markdown report (*.md extension)
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
