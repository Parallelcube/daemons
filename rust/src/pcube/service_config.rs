
const SYSTEM_HOST_QUEUE_NAME: &str = "/mq_queue_host";
const SYSTEM_WORKER_QUEUE_NAME: &str = "/mq_queue_worker";

pub struct ServiceConfig 
{
    pub is_host: bool,
    pub q_name_host: Option<String>,
    pub q_name_worker: Option<String>
}

impl ServiceConfig
{
    pub fn new(args: &mut Vec<String>) -> Self 
    {
        let mut service_config = Self {is_host: false, q_name_host: None, q_name_worker: None};
        service_config.match_is_host(args);
        if service_config.is_host
        {
            service_config.q_name_host = Some(SYSTEM_HOST_QUEUE_NAME.to_string());
            service_config.q_name_worker = Some(SYSTEM_WORKER_QUEUE_NAME.to_string());
        }
        else
        {
            service_config.q_name_host = Some(SYSTEM_WORKER_QUEUE_NAME.to_string());
            service_config.q_name_worker = Some(SYSTEM_HOST_QUEUE_NAME.to_string());
        }
        return service_config;
    }

    pub fn match_is_host(&mut self, args: &mut Vec<String>)
    {
        match args.iter().position(|n| n == "--host")
        {
            Some(position) => 
            {
                self.is_host=true;
                args.remove(position);
            },
            None => {}
        };
    }
}