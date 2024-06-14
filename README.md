# Source Code Scanner

Scans a specified project folder and outputs a summary:
- total number of files
- total sum of lines of code

<p align="center">
<img width="842" alt="shot" src="https://github.com/RustamIrzaev/source-code-scanner/assets/352703/89d54144-9c84-4a61-ae17-7375a7a0c69d">
</p>


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
