import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
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
    const transitioningUuid = ref<string | null>(null);

    // Modal and form state for adding new connections
    const showAddModal = ref(false);
    const formData = ref({
        mode: 'import', // 'import' or 'manual'
        type: 'openvpn', // 'openvpn', 'wireguard', 'openconnect', 'l2tp', 'pptp'
        name: '',
        filePath: '',
        gateway: '',
        username: '',
        password: '',
        psk: ''
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
                if (conn.uuid === transitioningUuid.value) return 2;
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
        if (conn.active || transitioningUuid.value) return;

        transitioningUuid.value = conn.uuid;
        try {
            await invoke('connect_vpn', { uuid: conn.uuid });
            showToast(`Connecting to ${conn.name}...`, 'info');
            
            // Poll until NM registers it as active
            let attempts = 0;
            let connected = false;
            while (attempts < 15) {
                await new Promise(resolve => setTimeout(resolve, 800));
                await fetchConnections();
                const updated = connections.value.find(c => c.uuid === conn.uuid);
                if (updated && updated.active) {
                    connected = true;
                    break;
                }
                attempts++;
            }
            if (connected) {
                showToast(`Connected to ${conn.name}`, 'success');
            } else {
                showToast(`Connection to ${conn.name} is taking longer than expected`, 'warning');
            }
        } catch (e) {
            showToast(`Failed to connect to ${conn.name}: ${e}`, 'error');
        } finally {
            transitioningUuid.value = null;
            fetchConnections();
        }
    };

    /**
     * Disconnects an active VPN connection.
     */
    const disconnect = async (conn: VpnConnection) => {
        if (!conn.active || transitioningUuid.value) return;

        transitioningUuid.value = conn.uuid;
        try {
            await invoke('disconnect_vpn', { uuid: conn.uuid });
            showToast(`Disconnecting from ${conn.name}...`, 'info');
            
            // Poll until NM registers it as inactive
            let attempts = 0;
            let disconnected = false;
            while (attempts < 15) {
                await new Promise(resolve => setTimeout(resolve, 800));
                await fetchConnections();
                const updated = connections.value.find(c => c.uuid === conn.uuid);
                if (updated && !updated.active) {
                    disconnected = true;
                    break;
                }
                attempts++;
            }
            if (disconnected) {
                showToast(`Disconnected from ${conn.name}`, 'success');
            } else {
                showToast(`Disconnection from ${conn.name} is taking longer than expected`, 'warning');
            }
        } catch (e) {
            showToast(`Failed to disconnect ${conn.name}: ${e}`, 'error');
        } finally {
            transitioningUuid.value = null;
            fetchConnections();
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
            mode: 'import',
            type: 'openvpn',
            name: '',
            filePath: '',
            gateway: '',
            username: '',
            password: '',
            psk: ''
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
        if (formData.value.mode === 'manual') {
            if (!formData.value.name.trim()) {
                showToast('Please enter a connection name', 'error');
                return;
            }
            if (!formData.value.gateway.trim()) {
                showToast('Please enter a gateway / server address', 'error');
                return;
            }
            try {
                await invoke('create_manual_vpn', {
                    name: formData.value.name,
                    vpnType: formData.value.type,
                    gateway: formData.value.gateway,
                    username: formData.value.username || null,
                    password: formData.value.password || null,
                    psk: formData.value.psk || null
                });

                showToast('VPN created successfully', 'success');
                showAddModal.value = false;
                await fetchConnections();
            } catch (e) {
                showToast('Failed to create VPN: ' + e, 'error');
            }
        } else {
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
        }
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

        if (showAddModal.value) return;

        if (sortedConnections.value.length === 0) return;

        if (e.key === 'ArrowDown') {
            selectedIndex.value = (selectedIndex.value + 1) % sortedConnections.value.length;
            e.preventDefault();
        } else if (e.key === 'ArrowUp') {
            selectedIndex.value = (selectedIndex.value - 1 + sortedConnections.value.length) % sortedConnections.value.length;
            e.preventDefault();
        } else if (e.key === 'Enter') {
            const conn = sortedConnections.value[selectedIndex.value];
            if (conn) {
                toggleConnection(conn);
            }
            e.preventDefault();
        }
    };

    watch(sortedConnections, (newVal) => {
        if (selectedIndex.value >= newVal.length) {
            selectedIndex.value = Math.max(0, newVal.length - 1);
        }
    });

    // --- Lifecycle ---
    let refreshInterval: ReturnType<typeof setInterval> | null = null;

    onMounted(async () => {
        loading.value = true;
        await fetchConnections();
        // Periodically refresh connection status
        refreshInterval = setInterval(fetchConnections, 5000);
        window.addEventListener('shortcut-add', openAddModal);
        window.addEventListener('keydown', handleListKeyDown);
    });

    onUnmounted(() => {
        if (refreshInterval) clearInterval(refreshInterval);
        window.removeEventListener('shortcut-add', openAddModal);
        window.removeEventListener('keydown', handleListKeyDown);
    });

    return {
        selectedIndex,
        connections,
        sortedConnections,
        loading,
        transitioningUuid,
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
