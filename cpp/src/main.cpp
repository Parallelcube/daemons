#include "Service.h"
#include "ServiceConfig.h"

#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <vector>

int main(int argc, char *argv[])
{
    std::vector<std::string> args(argv + 1, argv + argc);
    pcube::ServiceConfig service_config(args);
    pcube::Service service(service_config);
    int exit_code = service.run();
    return exit_code;
}