#pragma once

class CapabilityDropper {
public:
    CapabilityDropper();
    ~CapabilityDropper();

    void drop_all();
    void retain(unsigned long cap);

private:
    void apply();
};
