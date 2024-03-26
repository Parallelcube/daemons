mod service;
use std::process::ExitCode;

fn main() -> ExitCode
{
    let mut service = service::Service::new();
    let exit_code = match service.run() {
        service::EExitCode::SUCCESS => ExitCode::SUCCESS,
        service::EExitCode::FAIL => ExitCode::FAILURE,
    };
    ExitCode::from(exit_code)
}
