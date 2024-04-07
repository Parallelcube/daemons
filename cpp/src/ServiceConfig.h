#pragma once

#include <string>
#include <vector>


namespace pcube
{
    class ServiceConfig
    {
    public:
        ServiceConfig(std::vector<std::string>& args);
        virtual ~ServiceConfig();

        bool        is_master;
        std::string q_master_name;
        std::string q_slave_name;

    private:
        void match_is_master(std::vector<std::string>& args);

    };
}