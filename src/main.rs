extern crate vss_core;

use vss_core::communications::command_sender::CommandSender;
use vss_core::domain::team_type::TeamType;
use vss_core::domain::command::Command;
use vss_core::domain::wheels_command::WheelsCommand;
use vss_core::communications::state_receiver::StateReceiver;
use vss_core::domain::field_transaformation_type::FieldTransformationType;
use vss_core::communications::debug_sender::DebugSender;
use vss_core::domain::debug::Debug;
use vss_core::domain::pose::Pose;
use vss_core::domain::point::Point;
use vss_core::domain::state::State;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut command_sender = CommandSender::new_box()?;
    let mut debug_sender = DebugSender::new_box()?;
    let state_receiver = StateReceiver::new_box()?;

    command_sender.create_socket(TeamType::Yellow)?;
    debug_sender.create_socket(TeamType::Yellow)?;
    state_receiver.create_socket()?;

    let command = build_command();

    loop {
        let state = state_receiver.receive_state(FieldTransformationType::None)?;
        let debug = build_debug(state);
        command_sender.send_command(command.clone())?;
        debug_sender.send_debug(debug.clone())?;
    }
}

fn build_command() -> Command {
    let mut command = Command::new();

    command.commands = vec![
        WheelsCommand { left_vel: 10.0, right_vel: -10.0 },
        WheelsCommand { left_vel: 10.0, right_vel: -10.0 },
        WheelsCommand { left_vel: 10.0, right_vel: -10.0 }
    ];

    command
}

fn build_debug(state: State) -> Debug {
    let mut debug = Debug::new();

    debug.final_poses = state
        .team_yellow
        .iter()
        .map(|_| Pose {
            x: state.ball.x + 10.0,
            y: state.ball.y + 10.0,
            angle: 10.0,
        })
        .collect();

    debug.step_points = state
        .team_yellow
        .iter()
        .map(|pose| Point {
            x: pose.x + 10.0,
            y: pose.y + 10.0
        })
        .collect();

    debug
}