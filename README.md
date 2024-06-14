# Source Code Scanner

Scans a specified project folder and outputs a summary:
- total number of files
- total sum of lines of code

## Usage

`cargo run <folder_path> <file_extension>`

e.g. `cargo run /home/user/projects/ rs`

## Parameters
- `<folder_path>`: the path to the project folder
- `<file_extension>`: the file extension of the files to be scanned

## Ignored folders
At this moment, the list of ignored folders are hardcoded:
".idea", ".git", "node_modules", "obj", "bin", "build", "out", "libs".

However, you can change this directly from the code - see `exclude_folders` variable in `main.rs` file.

## License

The source code is available under the MIT license. See the [LICENSE](LICENSE) file for more info.
