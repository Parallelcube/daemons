use super::logger::log;
use super::enums::EExitCode;
use super::mq_handler::MQHandler;

pub struct Service 
{
    listening: bool,
    mq_handler: MQHandler
}

impl Service 
{
    pub fn new() -> Self 
    {
        Self {listening: false, mq_handler: MQHandler::new()}
    }

    pub fn start_listener(&mut self) -> bool 
    {
        self.listening = true;
        match self.mq_handler.connect("/mq_queue_slave", "/mq_queue_master")
        {
            EExitCode::SUCCESS => 
            {
                log("Service start listening");
                return true;
            }
            EExitCode::FAIL => 
            {
                return false;
            }
        }
    }

    pub fn stop_listener(&mut self) 
    {
        self.listening = false;
        log("Service stop listening");
        self.mq_handler.disconnect();
    }

    pub fn run(&mut self) -> EExitCode 
    {
        let mut exit_code = EExitCode::SUCCESS;
        if self.start_listener()
        {
            while self.listening
            {
                let (message, status) = self.mq_handler.receive_wait();
                match status
                {
                    EExitCode::SUCCESS => 
                    {
                        let message = format!("{} processed", message);
                        self.mq_handler.send_wait(message.as_str());
                        self.stop_listener();
                    }
                    EExitCode::FAIL => 
                    {
                        exit_code = EExitCode::FAIL;
                        self.stop_listener();
                    }
                }
            }
        }
        else
        {
            log("Unable to init listener");
            exit_code = EExitCode::FAIL;
        }
        return exit_code
    }
}