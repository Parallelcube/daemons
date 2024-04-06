class ServiceConfig:
    SYSTEM_QUEUE_MASTER_NAME = "/mq_queue_master"
    SYSTEM_QUEUE_SLAVE_NAME = "/mq_queue_slave"
    def __init__(self, is_master: bool=False):
        self.is_master = is_master
        if self.is_master:
            self.q_master_name = ServiceConfig.SYSTEM_QUEUE_MASTER_NAME
            self.q_slave_name = ServiceConfig.SYSTEM_QUEUE_SLAVE_NAME
        else:
            self.q_master_name = ServiceConfig.SYSTEM_QUEUE_SLAVE_NAME
            self.q_slave_name = ServiceConfig.SYSTEM_QUEUE_MASTER_NAME
