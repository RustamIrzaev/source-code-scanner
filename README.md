# Source Code Scanner

Scans a specified project folder and outputs a summary:
- total number of files
- total sum of lines of code

<p align="center">
<img width="842" alt="shot" src="https://github.com/RustamIrzaev/source-code-scanner/assets/352703/89d54144-9c84-4a61-ae17-7375a7a0c69d">
</p>


## Usage

Simple run:
`cargo run -- -f <folder_path> -e <file_extension>`

With ignored folders:
`cargo run -- -f <folder_path> -e <file_extension> -i <folder1>,<folder2>`

With hardcoded ignored folders:
`cargo run -- -f <folder_path> -e <file_extension> --hc`

Real example:
`cargo run -- -f /home/user/projects/my_project -e rs -i target,.git`

> Note. If you run the application not with `cargo run`, you should remove `--` from the argument.

## Parameters
- `-f` or `--folder`: the path to the project folder
- `-e` or `--extension`: the file extension of the files to be scanned
- `-i` or `--ignore`: the list of folders to be ignored, comma separated
- `--hc`: use hardcoded list of ignored folders _(see below)_. Please note that `-i` parameter has higher priority than this one
- `-V` or `--version`: prints the version of the program
- `-h` or `--help`: prints the help message

## Ignored folders
At this moment, the list of ignored folders are hardcoded:
".idea", ".git", "node_modules", "obj", "bin", "build", "out", "dist".

However, you can change this directly from the code - see `hardcoded.rs` file for a `EXCLUDE_FOLDERS_EMBEDDED` slice.

## License

The source code is available under the MIT license. See the [LICENSE](LICENSE) file for more info.
