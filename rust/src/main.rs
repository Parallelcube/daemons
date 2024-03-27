mod pcube;

use std::process::ExitCode;
use pcube::service::Service;
use pcube::service::EExitCode;

fn main() -> ExitCode
{
    let mut service = Service::new();
    let exit_code = match service.run() {
        EExitCode::SUCCESS => ExitCode::SUCCESS,
        EExitCode::FAIL => ExitCode::FAILURE,
    };
    ExitCode::from(exit_code)
}
