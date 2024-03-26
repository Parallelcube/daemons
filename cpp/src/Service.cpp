#include "Service.h"

#include <iostream>

using namespace pcube;

Service::Service():_listening(false)
{
}

Service::~Service()
{
}

int Service::run()
{
    int exit_code = EXIT_FAILURE;
    if (start_listener())
    {
        std::cout << "cpp: Service listening" << std::endl;
        exit_code = EXIT_SUCCESS;
    }
    else
    {
        std::cout << "cpp: Unable to init listener" << std::endl;
    }
    return exit_code;
}

bool Service::start_listener()
{
    _listening = true;
    return true;
}


void Service::stop_listener()
{
    _listening = false;
}