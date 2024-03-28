#pragma once

#include "MQHandler.h"

namespace pcube
{
    class Service
    {
    public:
        Service();
        virtual ~Service();

        int run();

    private:
        bool start_listener();
        void stop_listener();

        bool        _listening;
        MQHandler   _mq_handler;
    };
}