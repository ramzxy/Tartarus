#pragma once

#include <string>

class CgroupManager {
public:
    explicit CgroupManager(const std::string& cgroup_name);
    ~CgroupManager();

    void create();
    void set_memory_limit(size_t bytes);
    void set_cpu_limit(int percentage);
    void add_process(pid_t pid);

private:
    std::string cgroup_path_;
};
