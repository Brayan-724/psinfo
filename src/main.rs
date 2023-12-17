// use procfs::{process::Process};

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use procfs::process::Stat;

fn main() {
    let Ok(processes) = procfs::process::all_processes() else {
        eprintln!("Error getting processes");
        return;
    };

    let processes = processes
        .flatten()
        .map(|process| process.stat())
        .filter_map(Result::ok)
        .fold(
            HashMap::<PathBuf, Vec<Stat>>::new(),
            |mut processes, process| {
                let link_path = fs::read_link(format!("/proc/{}/cwd", process.pid))
                    .ok()
                    .or_else(|| Some(Path::new("No CWD").to_path_buf()))
                    .unwrap();

                let list = match processes.get_mut(&link_path) {
                    Some(a) => a,
                    None => {
                        processes.insert(link_path.clone(), Vec::new());
                        processes.get_mut(&link_path).unwrap()
                    }
                };
                list.push(process);

                processes
            },
        );

    for (process_path, processes) in processes {
        let processes = processes
            .iter()
            .map(|proc| format!("  [{pid}] {cmd}\n", pid = proc.pid, cmd = proc.comm))
            .collect::<String>();

        print!("{path}:\n{processes}", path = process_path.display());
    }
}
