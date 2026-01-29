/**
 * Wifi Model
 * Contains data structures for Wifi management
 */

export interface WifiNetwork {
    ssid: string;
    security: string;
    bars: string;
    active: boolean;
}

export interface WifiConfig {
    method: string;
    ip_address: string;
    prefix: number;
    gateway: string;
    dns: string;
}
