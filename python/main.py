import signal 
from pcube.service import Service
from pcube.logger import log

service = None

def cancel_callback(signum, frame):
    log(f'Signal {signum} reveived')
    if service:
        service.stop_listener()

signal.signal(signal.SIGTERM, cancel_callback)

if __name__ == "__main__":
    service = Service()
    exit_code = service.run()
    exit(exit_code.value)
