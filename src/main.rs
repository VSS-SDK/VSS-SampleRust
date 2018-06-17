extern crate vss_core_rust;

use std::{thread, time};
use self::vss_core_rust::communications::command_sender::CommandSender;
use vss_core_rust::domain::team_type::TeamType;
use vss_core_rust::domain::command::Command;
use vss_core_rust::domain::wheels_command::WheelsCommand;

fn main() {
    let mut command_sender = CommandSender::new();
    command_sender.create_socket(TeamType::Yellow);

    let mut command = Command::new();

    for i in 0..3 {
        command.commands.push(WheelsCommand {
            left_vel: 10.0,
            right_vel: -10.0
        });
    }

    loop {
        thread::sleep(time::Duration::from_millis(10));
        println!("{:?}", command);
        command_sender.send_command(command.clone());
    }
}