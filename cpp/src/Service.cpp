#include "Service.h"

#include "Logger.h"
#include "MQHandler.h"

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
        log("Service listening");
        exit_code = EXIT_SUCCESS;
    }
    else
    {
        log("Unable to init listener");
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