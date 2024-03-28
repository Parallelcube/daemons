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

bool Service::start_listener()
{
    _listening = true;
    int exit_code = _mq_handler.connect("/mq_queue_slave", "/mq_queue_master");
    log("Service start listening");
    return exit_code == EXIT_SUCCESS;
}

void Service::stop_listener()
{
    _listening = false;
    log("Service stop listening");
    _mq_handler.disconnect();
}

int Service::run()
{
    int exit_code = EXIT_SUCCESS;
    if (start_listener())
    {
        std::string message;
        while (_listening)
        {
            int status = _mq_handler.receive_wait(message);
            if (status == EXIT_SUCCESS)
            {
                message = message + " processed";
                _mq_handler.send_wait(message);
                stop_listener();
            }
            else
            {
                exit_code = EXIT_FAILURE;
                stop_listener();
            }
        }
    }
    else
    {
        log("Unable to init listener");
        exit_code = EXIT_FAILURE;
    }
    return exit_code;
}