#pragma once

#include <string>
#include <mqueue.h>

namespace pcube
{
    class MQHandler
    {
    public:
        MQHandler();
        virtual ~MQHandler();

        int connect(const std::string& mq_request_name, const std::string& mq_response_name);
        int disconnect();
        int send_wait(const std::string& message) const;
        int receive_wait(std::string& message) const;

    private:
        mqd_t _mq_request;
        mqd_t _mq_response;
    };
}