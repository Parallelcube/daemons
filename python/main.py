import sys
import signal 

from pcube.cli import Cli
from pcube.service_config import ServiceConfig
from pcube.service import Service
from pcube.logger import log

service = None

def cancel_callback(signum, frame):
    log(f'Signal {signum} reveived')
    if service:
        service.stop_listener()

signal.signal(signal.SIGTERM, cancel_callback)

if __name__ == "__main__":
    cli = Cli()
    cli.parse_args(sys.argv[1:])

    service = Service(ServiceConfig(cli.is_master))
    exit_code = service.run()
    sys.exit(exit_code.value)
