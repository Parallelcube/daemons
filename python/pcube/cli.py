class Cli:
    def __init__(self):
        self.is_master = False

    def check_is_master(self, args: list):
        for arg in args:
            if arg == "--master":
                self.is_master = True
                args.remove(arg)
                break
    
    def parse_args(self, args: list):
        self.check_is_master(args)