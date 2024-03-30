mod pcube;

use std::process::ExitCode;
use pcube::enums::EExitCode;
use pcube::service::Service;

fn main() -> ExitCode
{
    let mut service = Service::new();
    let exit_code = match service.run() {
        EExitCode::SUCCESS => ExitCode::SUCCESS,
        EExitCode::FAIL => ExitCode::FAILURE,
    };
    ExitCode::from(exit_code)
}
