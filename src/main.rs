use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread};
use std::time::Duration;

fn main() {
    // 读取命令文件
    let file_path = "commands.txt";
    let commands = read_commands_from_file(file_path);

    // 创建线程列表来管理每个命令的执行
    let mut handles = vec![];

    for command_line in commands {
        let command_line = command_line.to_string();
        let handle = thread::spawn(move || {
            execute_command(&command_line);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}

// 从文件中读取命令
fn read_commands_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Could not open file commands.txt");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("Could not read line")).collect()
}

// 执行单个命令并在失败时重试
fn execute_command(command_line: &str) {
    loop {
        println!("Executing command: {}", command_line);

        // 根据操作系统选择命令解释器
        let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        // 创建子进程
        let mut child = Command::new(shell)
            .arg(flag)
            .arg(command_line)
            .stdout(Stdio::inherit()) // 将子进程的输出继承到当前进程
            .stderr(Stdio::inherit()) // 将子进程的错误输出继承到当前进程
            .spawn();

        match child {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            println!("Command finished successfully. Restarting...");
                        } else {
                            println!("Command exited with status code: {}. Restarting...", status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to wait on child process: {}. Restarting...", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to start process: {}. Restarting...", e);
            }
        }

        // 等待一段时间后重试
        println!("Retrying in 5 seconds...");
        thread::sleep(Duration::from_secs(5));
    }
}
