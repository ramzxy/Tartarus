#pragma once

#include <stdexcept>
#include <unistd.h>

class FileDescriptor {
public:
    explicit FileDescriptor(int fd) : fd_(fd) {
        if (fd_ < 0) throw std::runtime_error("File descriptor creation failed");
    }

    ~FileDescriptor() { if (fd_ >= 0) close(fd_); }

    // moveable and not copyable
    FileDescriptor(FileDescriptor&& other) noexcept : fd_(other.fd_) { other.fd_ = -1; }
    FileDescriptor& operator=(FileDescriptor&& other) noexcept {
        if (this != &other) {
            if (fd_ >= 0) close(fd_);
            fd_ = other.fd_;
            other.fd_ = -1;
        }
        return *this;
    }

    FileDescriptor(const FileDescriptor&) = delete;
    FileDescriptor& operator=(const FileDescriptor&) = delete;

    int get() const { return fd_; }

private:
    int fd_;
};
