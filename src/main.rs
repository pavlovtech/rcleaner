use std::fs;
use std::env;  
use std::io;
use std::path::Path;

fn main() {
    let temp_dir = env::temp_dir().to_str().expect("err").to_string();

    let downloads_dir = get_os_var_plus_folder(&String::from("USERPROFILE"), &String::from("downloads"));

    let chrome_cache_dir = get_os_var_plus_folder(&String::from("LOCALAPPDATA"),
        &String::from("Google\\Chrome\\User Data\\Default\\Cache"));

    let chrome_code_cache_dir = get_os_var_plus_folder(&String::from("LOCALAPPDATA"),
        &String::from("Google\\Chrome\\User Data\\Default\\Code Cache"));

    println!("Removing chrome downloads");
    _ = remove_dir_contents(&downloads_dir);

    println!("Removing temp folder");
    _ = remove_dir_contents(&temp_dir);

    println!("Removing chrome cache");
    _ = remove_dir_contents(&chrome_cache_dir);

    println!("Removing code chrome cache");
    _ = remove_dir_contents(&chrome_code_cache_dir);

    io::stdin().read_line(&mut String::new()).unwrap();
}

fn get_os_var_plus_folder(var: &String, folder: &String) -> String {
    let dir: String = env::var_os(var)
        .expect("err")
        .to_str()
        .expect("err").to_string();

    let target_dir = dir + "\\" + folder;

    target_dir
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    println!("Getting dir");

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            remove_dir_contents(&path)?;
            println!("Removing dir {}", path.to_str().expect("Error removing dir"));
            _ = fs::remove_dir(path);
        } else {
            println!("Removing file {}", path.to_str().expect("Error removing file"));
            _ = fs::remove_file(path);
        }
    }
    Ok(())
}