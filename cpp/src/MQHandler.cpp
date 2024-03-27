#include "MQHandler.h"

#include "Logger.h"

#include <fcntl.h>

using namespace pcube;

constexpr mqd_t MQ_ERROR = (mqd_t)-1; 

MQHandler::MQHandler():_mq_request(MQ_ERROR),_mq_response(MQ_ERROR)
{
}

MQHandler::~MQHandler()
{
}

int MQHandler::connect(const std::string& mq_request_name, const std::string& mq_response_name)
{
    int exit_code = EXIT_SUCCESS;
    _mq_request = mq_open(mq_request_name.c_str(), O_RDONLY, 0600);
    if (_mq_request == MQ_ERROR)
    {
        log("Error mq_open with mq_request ("+mq_request_name+")");
        exit_code = EXIT_FAILURE;
    }

    if (!exit_code)
    {
        _mq_response = mq_open(mq_request_name.c_str(), O_RDONLY, 0600);
        if (_mq_response == MQ_ERROR)
        {
            log("Error mq_open with mq_response ("+mq_response_name+")");
            exit_code = EXIT_FAILURE;
        }
    }
    return exit_code;
}

int MQHandler::disconnect()
{
    int exit_code = EXIT_SUCCESS;
    if (_mq_request != MQ_ERROR)
    {
        if(mq_close(_mq_request) == MQ_ERROR)
        {
            log("Error mq_close with _mq_request");
            exit_code = EXIT_FAILURE;
        }
    }

    if (_mq_response != MQ_ERROR)
    {
        if(mq_close(_mq_response) == MQ_ERROR)
        {
            log("Error mq_close with _mq_response");
            exit_code = EXIT_FAILURE;
        }
    }

    return exit_code;
}

int MQHandler::send_wait(const char* buffer, size_t buffer_size) const
{
    int exit_code = EXIT_SUCCESS;
    int return_code = mq_send(_mq_response, buffer, buffer_size, 0);
    if (return_code == -1)
    {
        log("Error mq_send");
        exit_code = EXIT_FAILURE;
    }
    return exit_code;
}

int MQHandler::receive_wait(char* buffer, ssize_t& num_bytes, const size_t buffer_size) const
{
    int exit_code = EXIT_SUCCESS;
    num_bytes = mq_receive(_mq_request, buffer, buffer_size, 0);
    if (num_bytes == -1)
    {
        log("Error mq_receive");
        exit_code = EXIT_FAILURE;
    }
    return exit_code;
}