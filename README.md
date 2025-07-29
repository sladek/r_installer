**R_installer** is a small utility written in Rust designed to distribute and run R scripts as executables for specific operating systems, currently supporting only Windows and macOS. This tool can be particularly beneficial for individuals who do not have R installed or who are unfamiliar with the language.

The current version includes installers for Windows (R-4.5.1-win.exe) and macOS (R-4.5.1-arm64.pkg). However, it is currently fully functional only on Windows. Development of the macOS version is still ongoing due to unresolved permission issues.

R script that is "wrapped" into executable file should have a name "app.R". Otherwise the source code needs to be changed. The script needs to be placed to `data` directory. An example application is alredy present in this repository in `data` folder. Just replace it with your own and rebuild the application.

For testing, you can simply use the following commands: 

`cargo build`

or 

`cargo run`

commands.

For the release use

`cargo build --release`