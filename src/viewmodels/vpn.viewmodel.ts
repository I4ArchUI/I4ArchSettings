import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { VpnConnection } from '../models/vpn.model';
import { useToast } from '../composables/useToast';

/**
 * ViewModel for managing VPN connections.
 * Handles fetching, connecting, disconnecting, and importing VPN configurations.
 */
export function useVpnViewModel() {
    // --- State ---
    const connections = ref<VpnConnection[]>([]);
    const loading = ref(false);
    const connectingUuid = ref<string | null>(null);

    // Modal and form state for adding new connections
    const showAddModal = ref(false);
    const formData = ref({
        type: 'openvpn',
        filePath: '',
        username: '',
        password: ''
    });

    const { showToast } = useToast();

    // --- Computed ---

    /**
     * Connections sorted by status (Active > Connecting > Inactive) and then by name.
     */
    const sortedConnections = computed(() => {
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

    /**
     * Fetches the current list of VPN connections from the backend.
     */
    const fetchConnections = async () => {
        try {
            connections.value = await invoke('get_vpn_connections');
        } catch (e) {
            console.error("Failed to fetch VPNs:", e);
        } finally {
            loading.value = false;
        }
    };

    /**
     * Connects to a specific VPN connection.
     */
    const connect = async (conn: VpnConnection) => {
        if (conn.active || connectingUuid.value) return;

        connectingUuid.value = conn.uuid;
        try {
            await invoke('connect_vpn', { uuid: conn.uuid });
            showToast(`Connected to ${conn.name}`, 'success');
            conn.active = true;
            fetchConnections();
        } catch (e) {
            showToast(`Failed to connect to ${conn.name}: ${e}`, 'error');
        } finally {
            connectingUuid.value = null;
        }
    };

    /**
     * Disconnects an active VPN connection.
     */
    const disconnect = async (conn: VpnConnection) => {
        if (!conn.active) return;

        try {
            await invoke('disconnect_vpn', { uuid: conn.uuid });
            showToast(`Disconnected from ${conn.name}`, 'success');
            conn.active = false;
            fetchConnections();
        } catch (e) {
            showToast(`Failed to disconnect ${conn.name}: ${e}`, 'error');
        }
    };

    /**
     * Toggles the connection state (Connect/Disconnect).
     */
    const toggleConnection = async (conn: VpnConnection) => {
        if (conn.active) {
            await disconnect(conn);
        } else {
            await connect(conn);
        }
    };

    /**
     * Opens the modal to add a new VPN connection.
     */
    const openAddModal = () => {
        formData.value = {
            type: 'openvpn',
            filePath: '',
            username: '',
            password: ''
        };
        showAddModal.value = true;
    };

    /**
     * Opens a file picker to select a VPN configuration file.
     */
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

                    // Auto-detect VPN type based on file extension
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

    /**
     * Saves the current form data as a new VPN connection.
     */
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
        // Periodically refresh connection status
        refreshInterval = setInterval(fetchConnections, 5000);
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
