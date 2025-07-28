use std::fs::{self, exists, File, create_dir};
use std::io::Write;
use std::process::{Command};

fn main() {
    let installer: &[u8]; // This is a reference to specific installer
    // Installer array keeps the content of specific installer file so that it can be recreated if R isn't installed
    // Below we assign specific installer file based on operating system to installer.
    if cfg!(windows){
        let installer_file = include_bytes!("../data/R-4.5.1-win.exe");
        installer = installer_file;
    }
    else {
        let installer_file = include_bytes!("../data/R-4.5.1-arm64.pkg");
        installer = installer_file;
    }
    // app keeps the content of R application which is then recreated and executed
    let app = include_bytes!("../data/app.R");
    // Let's check if R is already installed
    println!("Checking if R is installed");
    let r_check = Command::new("R").arg("--version").output().unwrap();
    if r_check.status.success() {
        println!("R installed!");
//        exit(0); // This is temporary
    } else {
        {
            // Installer will be created in this block. Surrounding brackets {} are needed so that all the resources are released
            // and installer can be used via Command. At the end of the block "}" Drop is called and all the resources
            // are released. Othewise you would get and error "file is already used by different process" if you tried
            // to run inst_win.exe before you release (drop) the resources.
            println!("Creating R installer.");
            let mut file = File::create("inst_win.exe").expect("failed to open file");
            file.write_all(installer).expect("Cannot write to file");
        }
        println!("Running R installer");
        let _foo = Command::new("./inst_win.exe").arg("").output().unwrap();
        println!("Removing R installer.");
        let _ = fs::remove_file("./inst_win.exe");
    }
    // Now we can recreate the app.R script before we execute it. It is in the block {} so that all the resources
    // are released once the file is created and the execution leaves the block.
    {
        let mut file = File::create("app.R").expect("failed to open file");
        file.write_all(app).expect("Cannot write to file");         
    }
    // We will check if all the necessary directories exist. If not, we wil try to create them
    {
        match exists("data/libs") {
            Ok(true) => {
                println!("Path data/libs exists.");
            },
            Ok(false) => {
                println!("Path data/libs doesn't exist. Let's create it");
                let  _ = create_dir("data/libs");
            }
            Err(error) => {
                print!("Path cannot be verified: {:?}", error);
            }
        }
    }
    // Now we can try to run R script
    println!("Starting script");
//    let status = Command::new("Rscript.exe").arg("./app.R").status().expect("failed to execute child");
    let status = Command::new("Rscript.exe").arg("./app.R").status();
    println!("ExitStatus = {:?}", status)
}
