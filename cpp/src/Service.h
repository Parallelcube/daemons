#pragma once

#include "ServiceConfig.h"
#include "MQHandler.h"

namespace pcube
{
    class Service
    {
    public:
        Service(const ServiceConfig& config);
        virtual ~Service();

        int run();

    private:
        bool start_listener();
        void stop_listener();

        const ServiceConfig&    _config;
        bool                    _listening;
        MQHandler               _mq_handler;
    };
}