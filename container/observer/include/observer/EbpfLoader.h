#pragma once

#include <string>

class EbpfLoader {
public:
    EbpfLoader();
    ~EbpfLoader();

    void load_program(const std::string& path);
    void attach();
    void detach();

private:
    int prog_fd_;
};
