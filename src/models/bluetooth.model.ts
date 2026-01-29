/**
 * Bluetooth Model
 * Contains data structures for Bluetooth management
 */

export interface BluetoothDevice {
    mac: string;
    name: string;
    connected: boolean;
    paired: boolean;
    icon?: string;
}
