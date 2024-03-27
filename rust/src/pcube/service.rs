use super::{logger::log, mq_handler::MQHandler};

pub enum EExitCode 
{
    SUCCESS,
    FAIL,
}

pub struct Service 
{
    listening: bool,
    mq_handler: MQHandler
}

impl Service 
{
    pub fn new() -> Service 
    {
        Service {listening: false, mq_handler: MQHandler()}
    }

    pub fn run(&mut self) -> EExitCode 
    {
        let mut exit_code = EExitCode::FAIL;
        if self.start_listener()
        {
            log("Service listening");
            exit_code = EExitCode::SUCCESS
        }
        else
        {
            log("rust: Unable to init listener");
        }
        return exit_code
    }

    pub fn start_listener(&mut self) -> bool 
    {
        self.listening = true;
        return true;
    }

    pub fn stop_listener(&mut self) 
    {
        self.listening = false;
    }
}