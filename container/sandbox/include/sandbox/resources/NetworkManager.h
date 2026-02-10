#pragma once

class NetworkManager {
public:
    NetworkManager();
    ~NetworkManager();

    void setup_veth_pair();
    void configure_network_namespace();
    void apply_firewall_rules();
};
