#include "ServiceConfig.h"

#include <algorithm>
#include <vector>

using namespace pcube;

constexpr const char* SYSTEM_QUEUE_MASTER_NAME = "/mq_queue_master";
constexpr const char* SYSTEM_QUEUE_SLAVE_NAME = "/mq_queue_slave";

ServiceConfig::ServiceConfig(std::vector<std::string>& args):is_master(false)
{
    match_is_master(args);
    if (is_master)
    {
        q_master_name = SYSTEM_QUEUE_MASTER_NAME;
        q_slave_name = SYSTEM_QUEUE_SLAVE_NAME;
    }
    else
    {
        q_master_name = SYSTEM_QUEUE_SLAVE_NAME;
        q_slave_name = SYSTEM_QUEUE_MASTER_NAME;
    }
}

ServiceConfig::~ServiceConfig()
{
}

void ServiceConfig::match_is_master(std::vector<std::string>& args)
{
    auto it = std::find(args.begin(), args.end(), "--master");
    if (it != args.end())
    {
        is_master = true;
        it = args.erase(it);
    }
}
