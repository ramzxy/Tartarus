#pragma once

class NamespaceHandler {
public:
    NamespaceHandler();
    ~NamespaceHandler();

    void create_namespaces();
    void setup_user_namespace();

private:
    void unshare_namespaces();
};
