/**
 * Wifi Model
 * Contains data structures for Wifi management
 */

export interface WifiNetwork {
    ssid: string;
    security: string;
    bars: string;
    active: boolean;
    signal?: number;
}

export interface WifiConfig {
    method: string;
    ip_address: string;
    prefix: number;
    gateway: string;
    dns: string;
    bssid?: string;
    frequency?: string;
    speed?: string;
    interface?: string;
    mac_address?: string;
}
