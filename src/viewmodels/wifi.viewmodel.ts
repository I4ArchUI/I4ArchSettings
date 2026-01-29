import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { WifiNetwork, WifiConfig } from '../models/wifi.model.ts';
import { useToast } from '../composables/useToast';

/**
 * ViewModel for managing Wi-Fi networks and settings.
 * Handles scanning, connecting, and configuring Wi-Fi connections.
 */
export function useWifiViewModel() {
    // --- State ---
    const isEnabled = ref(false);
    const networks = ref<WifiNetwork[]>([]);
    const loading = ref(false);
    const connectingSsid = ref<string | null>(null);

    const { showToast } = useToast();

    // Configuration Modal state
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

    /**
     * Starts periodic background scanning for networks.
     */
    const startScanInterval = () => {
        if (scanInterval) clearInterval(scanInterval);
        scanInterval = setInterval(() => {
            if (isEnabled.value && !showConfigModal.value) {
                scan(true);
            }
        }, 10000);
    };

    /**
     * Stops the periodic scan interval.
     */
    const stopScanInterval = () => {
        if (scanInterval) {
            clearInterval(scanInterval);
            scanInterval = null;
        }
    };

    /**
     * Checks if Wi-Fi is currently enabled.
     */
    const checkStatus = async () => {
        try {
            isEnabled.value = await invoke('get_wifi_status');
        } catch (e) {
            console.error("Failed to check Wi-Fi status:", e);
        }
    };

    /**
     * Scans for available Wi-Fi networks.
     * @param isBackground If true, suppresses the loading spinner.
     */
    const scan = async (isBackground = false) => {
        if (!isBackground && networks.value.length === 0) {
            loading.value = true;
        }

        try {
            networks.value = await invoke('scan_wifi');
        } catch (e) {
            console.error("Wi-Fi scan failed:", e);
        } finally {
            loading.value = false;
        }
    };

    /**
     * Toggles Wi-Fi power on or off.
     */
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
            isEnabled.value = !isEnabled.value;
            showToast('Failed to toggle Wi-Fi', 'error');
        }
    };

    /**
     * Connects to a specific Wi-Fi network.
     */
    const connect = async (net: WifiNetwork) => {
        if (net.active || connectingSsid.value) return;

        connectingSsid.value = net.ssid;
        try {
            await invoke('connect_wifi', { ssid: net.ssid, password: null });
            await scan(true);
            showToast(`Connected to ${net.ssid}`, 'success');
        } catch (e: any) {
            showToast('Connection failed: ' + e, 'error');
        } finally {
            connectingSsid.value = null;
        }
    };

    /**
     * Opens the configuration modal for a specific network.
     */
    const openConfig = async (net: WifiNetwork) => {
        selectedSsid.value = net.ssid;
        try {
            const conf = await invoke<WifiConfig>('get_wifi_config', { ssid: net.ssid });
            config.value = conf;
            showConfigModal.value = true;
        } catch (e) {
            showToast('Failed to retrieve configuration', 'error');
        }
    };

    /**
     * Closes the configuration modal.
     */
    const closeConfig = () => {
        showConfigModal.value = false;
    };

    /**
     * Saves the current Wi-Fi configuration (IP, DNS, etc.) to the system.
     */
    const saveConfig = async () => {
        savingConfig.value = true;
        try {
            config.value.prefix = Number(config.value.prefix);
            await invoke('set_wifi_config', {
                ssid: selectedSsid.value,
                config: config.value
            });
            showConfigModal.value = false;
            await scan(true);
            showToast('Network settings saved successfully', 'success');
        } catch (e) {
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
        isEnabled,
        networks,
        loading,
        connectingSsid,
        showConfigModal,
        savingConfig,
        selectedSsid,
        config,
        toggleWifi,
        scan,
        connect,
        openConfig,
        closeConfig,
        saveConfig
    };
}
