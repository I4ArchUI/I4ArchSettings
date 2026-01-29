/**
 * VPN ViewModel (Composable)
 */

import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { VpnConnection } from '../models/vpn.model';
import { useToast } from '../composables/useToast';

export function useVpnViewModel() {
    // --- State ---
    const connections = ref<VpnConnection[]>([]);
    const loading = ref(false);

    // For tracking ongoing connection attempts
    const connectingUuid = ref<string | null>(null);

    // Add VPN Modal State
    const showAddModal = ref(false);
    const formData = ref({
        type: 'openvpn', // default
        filePath: '',
        username: '',
        password: ''
    });

    // Notifications
    const { showToast } = useToast();

    // --- Computed ---
    const sortedConnections = computed(() => {
        // Sort: Active first, then connecting, then name
        return [...connections.value].sort((a, b) => {
            const getRank = (conn: VpnConnection) => {
                if (conn.active) return 3;
                if (conn.uuid === connectingUuid.value) return 2;
                return 1;
            };

            const rankDiff = getRank(b) - getRank(a);
            if (rankDiff !== 0) return rankDiff;

            return a.name.localeCompare(b.name);
        });
    });

    // --- Actions ---

    const fetchConnections = async () => {
        // If initial load or not refreshed recently
        if (loading.value && connections.value.length === 0) {
            // keep loading true
        } else {
            // loading.value = true; // Optional: don't flicker loading on refresh
        }

        try {
            connections.value = await invoke('get_vpn_connections');
        } catch (e) {
            console.error("Failed to fetch VPNs:", e);
        } finally {
            loading.value = false;
        }
    };

    const connect = async (conn: VpnConnection) => {
        if (conn.active || connectingUuid.value) return;

        connectingUuid.value = conn.uuid;
        try {
            await invoke('connect_vpn', { uuid: conn.uuid });
            showToast(`Connected to ${conn.name}`, 'success');
            // Optimistic update or refresh
            conn.active = true;
            fetchConnections();
        } catch (e) {
            showToast(`Failed to connect to ${conn.name}: ${e}`, 'error');
        } finally {
            connectingUuid.value = null;
        }
    };

    const disconnect = async (conn: VpnConnection) => {
        if (!conn.active) return;

        // We could track disconnecting state similarly if needed
        try {
            await invoke('disconnect_vpn', { uuid: conn.uuid });
            showToast(`Disconnected from ${conn.name}`, 'success');
            conn.active = false;
            fetchConnections();
        } catch (e) {
            showToast(`Failed to disconnect ${conn.name}: ${e}`, 'error');
        }
    };

    const toggleConnection = async (conn: VpnConnection) => {
        if (conn.active) {
            await disconnect(conn);
        } else {
            await connect(conn);
        }
    };

    const openAddModal = () => {
        formData.value = {
            type: 'openvpn',
            filePath: '',
            username: '',
            password: ''
        };
        showAddModal.value = true;
    };

    const pickFile = async () => {
        try {
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'VPN Configuration',
                    extensions: ['ovpn', 'conf', 'wg']
                }]
            });

            if (selected) {
                const path = Array.isArray(selected) ? selected[0] : selected;
                if (path) {
                    formData.value.filePath = path;

                    // Auto-detect type
                    if (path.endsWith('.conf') || path.endsWith('.wg')) {
                        formData.value.type = 'wireguard';
                    } else if (path.endsWith('.ovpn')) {
                        formData.value.type = 'openvpn';
                    }
                }
            }
        } catch (e) {
            console.error(e);
        }
    };

    const saveConnection = async () => {
        if (!formData.value.filePath) {
            showToast('Please select a configuration file', 'error');
            return;
        }

        try {
            await invoke('import_vpn', {
                filePath: formData.value.filePath,
                vpnType: formData.value.type,
                username: formData.value.username || null,
                password: formData.value.password || null
            });

            showToast('VPN imported successfully', 'success');
            showAddModal.value = false;
            await fetchConnections();
        } catch (e) {
            showToast('Failed to import VPN: ' + e, 'error');
        }
    };

    // --- Lifecycle ---
    let refreshInterval: ReturnType<typeof setInterval> | null = null;

    onMounted(async () => {
        loading.value = true;
        await fetchConnections();

        refreshInterval = setInterval(fetchConnections, 5000); // Auto refresh status
    });

    onUnmounted(() => {
        if (refreshInterval) clearInterval(refreshInterval);
    });

    return {
        connections,
        sortedConnections,
        loading,
        connectingUuid,
        fetchConnections,
        connect,
        disconnect,
        toggleConnection,
        showAddModal,
        formData,
        openAddModal,
        pickFile,
        saveConnection
    };
}
