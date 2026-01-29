/**
 * Bluetooth ViewModel (Composable)
 * Contains business logic for Bluetooth management
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { BluetoothDevice } from '../models/bluetooth.model';
import { useToast } from '../composables/useToast';

export function useBluetoothViewModel() {
    // --- State ---
    const isEnabled = ref(false);
    const devices = ref<BluetoothDevice[]>([]);
    const loading = ref(false);

    // Notifications
    const { showToast } = useToast();

    // --- Actions ---

    const checkStatus = async () => {
        try {
            isEnabled.value = await invoke('get_bluetooth_status');
        } catch (e) {
            console.error('Failed to get status:', e);
        }
    };

    const scan = async () => {
        loading.value = true;
        try {
            devices.value = await invoke('scan_bluetooth');
        } catch (e) {
            console.error('Scan failed:', e);
            showToast('Bluetooth scan failed: ' + e, 'error');
        } finally {
            loading.value = false;
        }
    };

    const toggleBluetooth = async () => {
        try {
            await invoke('toggle_bluetooth', { enable: isEnabled.value });
            if (isEnabled.value) {
                scan();
            } else {
                devices.value = [];
            }
        } catch (e) {
            console.error('Toggle failed:', e);
            isEnabled.value = !isEnabled.value;
            showToast('Failed to toggle Bluetooth: ' + e, 'error');
        }
    };

    const connect = async (dev: BluetoothDevice) => {
        if (dev.connected) return;

        loading.value = true;
        try {
            await invoke('connect_bluetooth', { mac: dev.mac });
            // Ideally re-scan or update status, here we just optimistically set connected
            dev.connected = true;
            showToast(`Connected to ${dev.name || dev.mac}`, 'success');
        } catch (e) {
            console.error('Connect failed:', e);
            showToast(`Failed to connect to ${dev.name || dev.mac}: ` + e, 'error');
        } finally {
            loading.value = false;
        }
    };

    // --- Lifecycle ---
    onMounted(async () => {
        await checkStatus();
        if (isEnabled.value) {
            scan();
        }
    });

    return {
        isEnabled,
        devices,
        loading,
        toggleBluetooth,
        connect,
        scan
    };
}
