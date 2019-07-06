use std::{fs, env, process};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (dir, take_every) = match args.len() {
        1 | 2 => {
            println!("Arguments: <Directory> <Take Every>");
            process::exit(0);
        },
        3 | _ => {
            (args[1].as_str(), args[2].parse().unwrap())
        }
    };

    let mut new_dir = PathBuf::from(dir);
    new_dir.push(format!("every_{}", take_every));
    if !new_dir.exists() {
        fs::create_dir(&new_dir).expect("Unable to create directory");
    }

    let mut files = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .collect::<Vec<_>>();

    files.sort();

    println!("Total files: {}", files.len());

    let every_x_files = files
        .iter()
        .step_by(take_every)
        .collect::<Vec<_>>();

    let new_files = every_x_files.iter()
        .map(|file| {
            let mut new_file = new_dir.to_path_buf();
            new_file.push(file.file_name().unwrap());
            new_file
        })
//        .inspect(|file| println!("{}", file.to_str().unwrap()))
        .collect::<Vec<_>>();
    println!("Filtered files: {}", every_x_files.len());

    every_x_files.iter().zip(new_files.iter()).for_each(|(original, new)| {
        if !new.exists() {
//            println!("Symlink of {} to {}", original.to_str().unwrap(), new.to_str().unwrap());
            std::os::unix::fs::symlink(original, new).expect(&format!("Unable to create symlink to {}", new.to_str().unwrap()));
        }
    });
}

