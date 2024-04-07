#include "Service.h"

#include "Logger.h"
#include "MQHandler.h"

#include <iostream>

using namespace pcube;

Service::Service(const ServiceConfig& config):  _config(config),
                                                _listening(false)
{
}

Service::~Service()
{
}

std::string bool_to_string(bool value) 
{
    return value ? "true" : "false";
}

bool Service::start_listener()
{
    _listening = true;
    int exit_code = _mq_handler.connect(_config.q_master_name, _config.q_slave_name);
    if (exit_code == EXIT_SUCCESS)
    {
        log("Service start listening : master("+bool_to_string(_config.is_master)+")");
        return true;
    }
    return false;
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
        if (_config.is_master)
        {
            _mq_handler.send_wait("task-1");
        }

        std::string message;
        while (_listening)
        {
            int status = _mq_handler.receive_wait(message);
            if (status == EXIT_SUCCESS)
            {
                if (!_config.is_master)
                {
                    message = message + " processed";
                    _mq_handler.send_wait(message);
                }
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