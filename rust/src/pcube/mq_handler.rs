extern crate nix;

use super::logger::log;
use nix::mqueue::{MqdT, mq_open, mq_close, mq_send, mq_receive, MQ_OFlag, mq_attr_member_t};
use nix::sys::stat::Mode;

pub struct MQHandler 
{
    mq_request: MqdT,
    mq_response: MqdT,
}

impl MQHandler 
{
    pub fn new() -> MQHandler 
    {
        MQHandler {mq_request: MqdT(-1), mq_response: MqdT(-1)}
    }

    pub fn connect(&mut self, mq_request_name: &str, mq_response_name: &str) -> i32 
    {
        log("connect");
        const MSG_SIZE: mq_attr_member_t = 32;

        let oflag0 = MQ_OFlag::O_CREAT | MQ_OFlag::O_WRONLY;
        let mode = Mode::S_IWUSR | Mode::S_IRUSR | Mode::S_IRGRP | Mode::S_IROTH;
        self.mq_request = mq_open(mq_request_name, oflag0, mode, None).unwrap();

        let oflag1 = MQ_OFlag::O_CREAT | MQ_OFlag::O_RDONLY;
        self.mq_response = mq_open(mq_response_name, oflag1, mode, None).unwrap();
        return 0;
    }

    pub fn disconnect(&mut self) -> i32 
    {
        mq_close(self.mq_request).unwrap();
        mq_close(self.mq_response).unwrap();
        return 0;
    }

    pub fn send_wait(&mut self) -> i32 
    {
        let msg_to_send = b"msg_1";
        mq_send(&self.mq_request, msg_to_send, 1).unwrap();
        return 0;
    }

    pub fn receive_wait(&mut self) -> i32 
    {
        let mut buf = [0u8; 32];
        let mut prio = 0u32;
        let len = mq_receive(&self.mq_response, &mut buf, &mut prio).unwrap();
        return 0;
    }
}