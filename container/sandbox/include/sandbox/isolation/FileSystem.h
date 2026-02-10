#pragma once

#include <string>

class FileSystem {
public:
    explicit FileSystem(const std::string& root_path);
    ~FileSystem();

    void setup_rootfs();
    void mount_proc();
    void pivot_root();

private:
    std::string root_path_;
};
