from enum import Enum

class EExitCode(Enum):
    SUCCESS = 0
    FAIL = 1

class Service:
    def __init__(self):
        self._listening = False

    def start_listener(self) -> bool:
        self._listening = True
        return True

    def stop_listener(self):
        self._listening = False

    def run(self) -> EExitCode:
        exit_code = EExitCode.FAIL
        if self.start_listener():
            print("python: Service listening")
            exit_code = EExitCode.SUCCESS
        else:
            print("python: Unable to init listener")
        return exit_code
        