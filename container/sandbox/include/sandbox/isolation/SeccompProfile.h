#pragma once

class SeccompProfile {
public:
    SeccompProfile();
    ~SeccompProfile();

    void load_default_profile();
    void allow_syscall(int syscall_nr);
    void apply();

private:
    void build_filter();
};
