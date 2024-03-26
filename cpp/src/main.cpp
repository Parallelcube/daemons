#include "Service.h"

#include <cstdlib>
#include <iomanip>
#include <iostream>

int main(int argc, char *argv[])
{
    pcube::Service service;
    int exit_code = service.run();
    return exit_code;
}