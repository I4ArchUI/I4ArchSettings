/**
 * Bluetooth ViewModel (Composable)
 * Contains business logic for Bluetooth management
 */

import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { BluetoothDevice } from '../models/bluetooth.model';
import { useToast } from '../composables/useToast';

export function useBluetoothViewModel() {
    // --- State ---
    const isEnabled = ref(false);
    const devices = ref<BluetoothDevice[]>([]);
    const loading = ref(false);
    const connectingMac = ref<string | null>(null);
    const localName = ref("Arch Linux PC");

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
            console.error("Failed to check bluetooth status:", e);
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
            console.error("Failed to fetch bluetooth devices:", e);
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

    const triggerToggle = () => {
        isEnabled.value = !isEnabled.value;
        toggleBluetooth();
    };

    const selectedIndex = ref(0);

    const handleListKeyDown = (e: KeyboardEvent) => {
        const activeElement = document.activeElement;
        const isTyping = activeElement && (
            activeElement.tagName === 'INPUT' ||
            activeElement.tagName === 'TEXTAREA' ||
            activeElement.tagName === 'SELECT' ||
            activeElement.getAttribute('contenteditable') === 'true'
        );
        if (isTyping) return;

        if (!isEnabled.value || sortedDevices.value.length === 0) return;

        if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {
            selectedIndex.value = (selectedIndex.value + 1) % sortedDevices.value.length;
            e.preventDefault();
        } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
            selectedIndex.value = (selectedIndex.value - 1 + sortedDevices.value.length) % sortedDevices.value.length;
            e.preventDefault();
        } else if (e.key === 'Enter') {
            const dev = sortedDevices.value[selectedIndex.value];
            if (dev) {
                connect(dev);
            }
            e.preventDefault();
        }
    };

    watch(sortedDevices, (newVal) => {
        if (selectedIndex.value >= newVal.length) {
            selectedIndex.value = Math.max(0, newVal.length - 1);
        }
    });

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
        try {
            localName.value = await invoke('get_local_adapter_name');
        } catch (e) {
            console.error("Failed to get local adapter name:", e);
        }
        if (isEnabled.value) {
            await startScan();
            refreshDevices();
            startRefreshInterval();
        }
        window.addEventListener('shortcut-toggle', triggerToggle);
        window.addEventListener('keydown', handleListKeyDown);
    });

    onUnmounted(() => {
        stopRefreshInterval();
        stopScan();
        window.removeEventListener('shortcut-toggle', triggerToggle);
        window.removeEventListener('keydown', handleListKeyDown);
    });

    return {
        selectedIndex,
        isEnabled,
        devices,
        loading,
        toggleBluetooth,
        connect,
        connectingMac,
        sortedDevices,
        localName,
        scan: refreshDevices // exposing as 'scan' for backward compatibility
    };
}
