#pragma once

#include <string>

class ResourceManager {
public:
    ResourceManager();
    ~ResourceManager();

    void set_limits();
    void apply(pid_t pid);

private:
    CgroupManager* cgroup_;
};
