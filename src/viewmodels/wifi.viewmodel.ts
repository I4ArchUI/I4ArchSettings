/**
 * Wifi ViewModel (Composable)
 * Contains business logic for Wifi management
 */

import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { WifiNetwork, WifiConfig } from '../models/wifi.model.ts';
import { useToast } from '../composables/useToast';

export function useWifiViewModel() {
    // --- State ---
    const isEnabled = ref(false);
    const networks = ref<WifiNetwork[]>([]);
    const loading = ref(false);
    const connectingSsid = ref<string | null>(null);

    // Notifications
    const { showToast } = useToast();

    // Config Modal State
    const showConfigModal = ref(false);
    const savingConfig = ref(false);
    const selectedSsid = ref('');
    const config = ref<WifiConfig>({
        method: 'auto',
        ip_address: '',
        prefix: 24,
        gateway: '',
        dns: ''
    });

    let scanInterval: ReturnType<typeof setInterval> | null = null;

    // --- Actions ---

    const startScanInterval = () => {
        if (scanInterval) clearInterval(scanInterval);
        scanInterval = setInterval(() => {
            if (isEnabled.value && !showConfigModal.value) {
                scan(true);
            }
        }, 10000);
    };

    const stopScanInterval = () => {
        if (scanInterval) {
            clearInterval(scanInterval);
            scanInterval = null;
        }
    };

    const checkStatus = async () => {
        try {
            isEnabled.value = await invoke('get_wifi_status');
        } catch (e) {
            console.error('Failed to check status:', e);
        }
    };

    const scan = async (isBackground = false) => {
        if (!isBackground && networks.value.length === 0) {
            loading.value = true;
        }

        try {
            networks.value = await invoke('scan_wifi');
        } catch (e) {
            console.error('Scan failed:', e);
        } finally {
            loading.value = false;
        }
    };

    const toggleWifi = async () => {
        try {
            await invoke('toggle_wifi', { enable: isEnabled.value });
            if (isEnabled.value) {
                await scan(false);
                startScanInterval();
            } else {
                networks.value = [];
                stopScanInterval();
            }
        } catch (e) {
            console.error('Toggle failed:', e);
            // Revert state on failure
            isEnabled.value = !isEnabled.value;
            showToast('Failed to toggle Wi-Fi', 'error');
        }
    };

    const connect = async (net: WifiNetwork) => {
        // Prevent clicking active or processing network
        if (net.active || connectingSsid.value) return;

        connectingSsid.value = net.ssid;

        try {
            // "connect_wifi" calls nmcli (blocking)
            await invoke('connect_wifi', { ssid: net.ssid, password: null });

            // Re-scan to update active status
            await scan(true);
            showToast(`Connected to ${net.ssid}`, 'success');
        } catch (e: any) {
            console.error('Connection failed:', e);
            showToast('Connection failed: ' + e, 'error');
        } finally {
            connectingSsid.value = null;
        }
    };

    const openConfig = async (net: WifiNetwork) => {
        selectedSsid.value = net.ssid;
        try {
            const conf = await invoke<WifiConfig>('get_wifi_config', { ssid: net.ssid });
            config.value = conf;
            showConfigModal.value = true;
        } catch (e) {
            console.error('Failed to get config', e);
            showToast('Failed to retrieve configuration', 'error');
        }
    };

    const closeConfig = () => {
        showConfigModal.value = false;
    };

    const saveConfig = async () => {
        savingConfig.value = true;
        try {
            // Ensure prefix is a number just in case
            config.value.prefix = Number(config.value.prefix);

            await invoke('set_wifi_config', {
                ssid: selectedSsid.value,
                config: config.value
            });

            showConfigModal.value = false;
            // Re-scan to reflect changes if necessary
            await scan(true);
            showToast('Network settings saved successfully', 'success');
        } catch (e) {
            console.error('Failed to save config', e);
            showToast(`Failed to apply settings: ${e}`, 'error');
        } finally {
            savingConfig.value = false;
        }
    };

    // --- Lifecycle ---
    onMounted(async () => {
        await checkStatus();
        if (isEnabled.value) {
            await scan(false);
            startScanInterval();
        }
    });

    onUnmounted(() => {
        stopScanInterval();
    });

    return {
        // State
        isEnabled,
        networks,
        loading,
        connectingSsid,
        showConfigModal,
        savingConfig,
        selectedSsid,
        config,

        // Methods
        toggleWifi,
        scan,
        connect,
        openConfig,
        closeConfig,
        saveConfig
    };
}
