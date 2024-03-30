from pcube.logger import log
from pcube.enums import EExitCode
from pcube.mq_handler import MQHandler

class Service:
    def __init__(self):
        self._listening = False
        self._mq_handler = MQHandler()

    def start_listener(self) -> bool:
        self._listening = True
        exit_code = self._mq_handler.connect("/mq_queue_master", "/mq_queue_slave")
        if exit_code == EExitCode.SUCCESS:
            log("Service start listening")
            return True
        return False

    def stop_listener(self):
        self._listening = False
        log("Service stop listening")
        self._mq_handler.disconnect()

    def run(self) -> EExitCode:
        exit_code = EExitCode.SUCCESS
        if self.start_listener():
            self._mq_handler.send_wait("task-1")
            while self._listening:
                message, status = self._mq_handler.receive_wait()
                if status == EExitCode.SUCCESS:
                    self.stop_listener()
                else:
                    exit_code = EExitCode.FAIL
                    self.stop_listener()
        else:
            log("Unable to init listener")
            exit_code = EExitCode.FAIL
        return exit_code
        