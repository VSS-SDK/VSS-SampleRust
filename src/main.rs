extern crate vss_core_rust;

use vss_core_rust::communications::command_sender::CommandSender;
use vss_core_rust::domain::team_type::TeamType;
use vss_core_rust::domain::command::Command;
use vss_core_rust::domain::wheels_command::WheelsCommand;
use vss_core_rust::communications::state_receiver::StateReceiver;
use vss_core_rust::domain::field_transaformation_type::FieldTransformationType;

fn main() {
    let mut command_sender = CommandSender::new();
    let mut state_receiver = StateReceiver::new();

    command_sender.create_socket(TeamType::Yellow);
    state_receiver.create_socket();

    let mut command = Command::new();

    for i in 0..3 {
        command.commands.push(WheelsCommand {
            left_vel: 10.0,
            right_vel: -10.0
        });
    }

    loop {
        let state = state_receiver.receive_state(FieldTransformationType::None);
        command_sender.send_command(command.clone());
    }
}