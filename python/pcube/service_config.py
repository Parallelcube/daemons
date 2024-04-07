class ServiceConfig:
    SYSTEM_QUEUE_MASTER_NAME = "/mq_queue_master"
    SYSTEM_QUEUE_SLAVE_NAME = "/mq_queue_slave"
    def __init__(self, args: list):
        self.is_master = False
        self._match_is_master(args)
        if self.is_master:
            self.q_master_name = ServiceConfig.SYSTEM_QUEUE_MASTER_NAME
            self.q_slave_name = ServiceConfig.SYSTEM_QUEUE_SLAVE_NAME
        else:
            self.q_master_name = ServiceConfig.SYSTEM_QUEUE_SLAVE_NAME
            self.q_slave_name = ServiceConfig.SYSTEM_QUEUE_MASTER_NAME

    def _match_is_master(self, args: list):
        for arg in args:
            if arg == "--master":
                self.is_master = True
                args.remove(arg)
                break
