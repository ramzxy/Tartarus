#pragma once

class EventReader {
public:
    EventReader();
    ~EventReader();

    void start();
    void stop();

private:
    void poll_events();
    bool running_;
};
