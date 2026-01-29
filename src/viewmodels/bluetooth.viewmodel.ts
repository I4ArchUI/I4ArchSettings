/**
 * Bluetooth ViewModel (Composable)
 * Contains business logic for Bluetooth management
 */

import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { BluetoothDevice } from '../models/bluetooth.model';
import { useToast } from '../composables/useToast';

export function useBluetoothViewModel() {
    // --- State ---
    const isEnabled = ref(false);
    const devices = ref<BluetoothDevice[]>([]);
    const loading = ref(false);
    const connectingMac = ref<string | null>(null);

    // Computed property for sorted devices: Connected > Connecting > Disconnected
    const sortedDevices = computed(() => {
        return [...devices.value].sort((a, b) => {
            const getRank = (device: BluetoothDevice) => {
                if (device.connected) return 3;
                if (device.mac === connectingMac.value) return 2;
                return 1;
            };
            return getRank(b) - getRank(a);
        });
    });

    // Notifications
    const { showToast } = useToast();

    // --- Actions ---

    const checkStatus = async () => {
        try {
            isEnabled.value = await invoke('get_bluetooth_status');
        } catch (e) {
        }
    };

    const startScan = async () => {
        try {
            await invoke('start_scan');
        } catch (e) {
            console.error("Failed to start scan:", e);
        }
    };

    const stopScan = async () => {
        try {
            await invoke('stop_scan');
        } catch (e) {
            console.error("Failed to stop scan:", e);
        }
    };

    const refreshDevices = async () => {
        if (!isEnabled.value) return;

        if (loading.value) return;

        if (devices.value.length === 0) loading.value = true;

        try {
            devices.value = await invoke('get_bluetooth_devices');
        } catch (e) {
        } finally {
            loading.value = false;
        }
    };

    const toggleBluetooth = async () => {
        try {
            await invoke('toggle_bluetooth', { enable: isEnabled.value });
            if (isEnabled.value) {
                await startScan();
                refreshDevices();
                startRefreshInterval();
            } else {
                stopScan();
                stopRefreshInterval();
                devices.value = [];
            }
        } catch (e) {
            isEnabled.value = !isEnabled.value;
            showToast('Failed to toggle Bluetooth: ' + e, 'error');
        }
    };

    const connect = async (dev: BluetoothDevice) => {
        if (dev.connected || connectingMac.value) return;

        connectingMac.value = dev.mac;
        try {
            await invoke('connect_bluetooth', { mac: dev.mac });

            // Update device status in the local list immediately
            const device = devices.value.find(d => d.mac === dev.mac);
            if (device) device.connected = true;

            showToast(`Connected to ${dev.name || dev.mac}`, 'success');
        } catch (e) {
            showToast(`Failed to connect to ${dev.name || dev.mac}: ` + e, 'error');
        } finally {
            connectingMac.value = null;
        }
    };

    // --- Lifecycle ---
    let scanInterval: ReturnType<typeof setInterval> | null = null;

    const startRefreshInterval = () => {
        if (scanInterval) clearInterval(scanInterval);
        scanInterval = setInterval(refreshDevices, 15000);
    };

    const stopRefreshInterval = () => {
        if (scanInterval) {
            clearInterval(scanInterval);
            scanInterval = null;
        }
    };

    onMounted(async () => {
        await checkStatus();
        if (isEnabled.value) {
            await startScan();
            refreshDevices();
            startRefreshInterval();
        }
    });

    onUnmounted(() => {
        stopRefreshInterval();
        stopScan();
    });

    return {
        isEnabled,
        devices,
        loading,
        toggleBluetooth,
        connect,
        connectingMac,
        sortedDevices,
        scan: refreshDevices // exposing as 'scan' for backward compatibility if template uses it, though we should update template
    };
}
