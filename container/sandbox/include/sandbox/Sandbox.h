#pragma once

class Sandbox {
public:
    Sandbox();
    ~Sandbox();

    void setup();
    void run(const char* executable);

private:
    void apply_isolation();
    void apply_resource_limits();
};
