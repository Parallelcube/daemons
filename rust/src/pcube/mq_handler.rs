extern crate nix;

use super::logger::log;
use super::enums::EExitCode;
use nix::mqueue::{MqdT, mq_open, mq_close, mq_send, mq_receive, MQ_OFlag, MqAttr};
use nix::sys::stat::Mode;
use std::borrow::Borrow;
use std::str;

const MAX_MESSAGE_SIZE: i64 = 512;

pub struct MQHandler 
{
    mq_request: Option<MqdT>,
    mq_response: Option<MqdT>,
}

impl MQHandler 
{
    pub fn new() -> Self 
    {
        Self {mq_request: None, mq_response: None}
    }

    pub fn connect(&mut self, mq_request_name: &str, mq_response_name: &str) -> EExitCode 
    {
        let mut exit_code = EExitCode::SUCCESS;
        let mq_attributes = MqAttr::new(0, 1, MAX_MESSAGE_SIZE, 0);
        let mq_request_flags = MQ_OFlag::O_CREAT | MQ_OFlag::O_WRONLY;
        let mode = Mode::S_IWUSR | Mode::S_IRUSR;
        match mq_open(mq_request_name, mq_request_flags, mode, Some(&mq_attributes))
        {
            Ok(queue) => 
            {
                self.mq_request = Some(queue)
            }
            Err(error) => 
            {
                log(&format!("Error opening with mq_request {}", error));
                exit_code = EExitCode::FAIL
            }
        }

        let mq_response_flags = MQ_OFlag::O_CREAT | MQ_OFlag::O_RDONLY;
        match mq_open(mq_response_name, mq_response_flags, mode, Some(&mq_attributes))
        {
            Ok(queue) => 
            {
                self.mq_response = Some(queue)
            }
            Err(error) => 
            {
                log(&format!("Error opening with mq_response {}", error));
                exit_code = EExitCode::FAIL
            }
        }
        return exit_code;
    }

    pub fn disconnect(&mut self) -> EExitCode 
    {
        let exit_code = EExitCode::SUCCESS;
        if self.mq_request.is_some()
        {
            match mq_close(self.mq_request)
            {
                Ok(_) => {}
                Err(_) => {}
            }
            self.mq_request = None;
        }
        // if self.mq_response.is_some()
        // {
        //     mq_close(self.mq_response.unwrap()).unwrap();
        //     self.mq_response = None;
        // }
        return exit_code;
    }

    pub fn send_wait(&mut self, message: &str) -> EExitCode 
    {
        let mut exit_code = EExitCode::SUCCESS;
        log(&format!("Sending message '{}'", message));
        match mq_send(&self.mq_request.as_ref().unwrap(), message.as_bytes(), 1)
        {
            Ok(_) => {}
            Err(error) => 
            {
                log(&format!("Error mq_send '{}'", error));
                exit_code = EExitCode::FAIL;
            }
        }
        return exit_code;
    }

    pub fn receive_wait(&mut self) -> (String, EExitCode)
    {
        let mut buf = [0u8; MAX_MESSAGE_SIZE as usize];
        let mut priority = 0u32;
        match mq_receive(&self.mq_response.as_mut().unwrap(), &mut buf, &mut priority)
        {
            Ok(num_bytes) => 
            {
                let message = str::from_utf8(&buf[0..num_bytes]).unwrap();
                log(&format!("Received message '{}'", message));
                return (message.to_string(), EExitCode::SUCCESS)
            }
            Err(error) => 
            {
                log(&format!("Error mq_receive '{}'", error));
                return (error.to_string(), EExitCode::FAIL)
            }
        }
    }
}