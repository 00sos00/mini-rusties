use std::io::stdin;

fn handle_command(command: &str, arguments: Vec<&str>) {
    println!("Command: {} Args: {:?}", command, arguments);
}

fn main() {
    loop {
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        let command_and_args = command.trim().split(' ').collect::<Vec<&str>>();
        let command = command_and_args[0];
        let arguments = command_and_args[1..command_and_args.len()].to_vec();

        handle_command(command, arguments);
    }
}
