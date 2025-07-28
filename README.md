**R_installer** is a small utility written in rust that can help to ditribute and run R scripts as executable for specific operating syster (currently only windows and mac_os are supported). It can be useful for people who don't have installed R or are not familiar with this language.

Usage: Current version includes installers for windows (R-4.5.1-win.exe) and Mac OS (R-4.5.1-arm64.pkg), but it is currently fully functional only for windows. Work on Mac OS part is still in progress as there are some permition issues that need to be solved.

R script that is "wrapped" into executable file should have a name "app.R". Otherwise the source code needs to be changed. The script needs to be placed to `data` directory. An example application is alredy present in this repository in `data` folder. Just replace it with your own and rebuild the application.

For testing you can use just 

`cargo build`

or 

`cargo run`

commands.

For release use

`cargo build --release`