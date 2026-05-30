<script setup lang="ts">
import { useVpnViewModel } from '../viewmodels/vpn.viewmodel';
import LoadingState from '@/components/LoadingState.vue';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';
import ModalDialog from '@/components/common/ModalDialog.vue';

const {
    selectedIndex,
    sortedConnections,
    loading,
    transitioningUuid,
    toggleConnection,
    showAddModal,
    formData,
    openAddModal,
    pickFile,
    saveConnection
} = useVpnViewModel();

/**
 * Returns dynamic, harmonized color schemes and icons for each VPN type.
 */
const getVpnTypeDetails = (typeName: string) => {
    const type = (typeName || '').toLowerCase();
    if (type.includes('wireguard')) {
        return { 
            icon: 'pi pi-bolt', 
            bg: 'rgba(255, 179, 0, 0.15)', 
            border: 'rgba(255, 179, 0, 0.25)',
            color: '#ffb300', 
            label: 'WireGuard' 
        };
    } else if (type.includes('openvpn')) {
        return { 
            icon: 'pi pi-lock', 
            bg: 'rgba(41, 182, 246, 0.15)', 
            border: 'rgba(41, 182, 246, 0.25)',
            color: '#29b6f6', 
            label: 'OpenVPN' 
        };
    } else if (type.includes('openconnect') || type.includes('cisco')) {
        return { 
            icon: 'pi pi-server', 
            bg: 'rgba(171, 71, 188, 0.15)', 
            border: 'rgba(171, 71, 188, 0.25)',
            color: '#ab47bc', 
            label: 'Cisco' 
        };
    } else if (type.includes('l2tp')) {
        return { 
            icon: 'pi pi-shield', 
            bg: 'rgba(102, 187, 106, 0.15)', 
            border: 'rgba(102, 187, 106, 0.25)',
            color: '#66bb6a', 
            label: 'L2TP/IPsec' 
        };
    } else if (type.includes('pptp')) {
        return { 
            icon: 'pi pi-key', 
            bg: 'rgba(255, 167, 38, 0.15)', 
            border: 'rgba(255, 167, 38, 0.25)',
            color: '#ffa726', 
            label: 'PPTP' 
        };
    }
    return { 
        icon: 'pi pi-shield', 
        bg: 'rgba(144, 164, 174, 0.15)', 
        border: 'rgba(144, 164, 174, 0.25)',
        color: '#90a4ae', 
        label: typeName 
    };
};
</script>

<template>
  <PageLayout>
    <template #title>VPN</template>

    <template #actions>
        <div style="display: flex; align-items: center; gap: 8px;">
            <button class="action-btn" @click="openAddModal">
                <i class="pi pi-plus" style="margin-right: 6px;"></i>
                Add Connection
            </button>
            <span class="kbd-hint"><kbd>Alt</kbd>+<kbd>N</kbd></span>
        </div>
    </template>
    
    <div class="vpn-list">
        <LoadingState v-if="loading && sortedConnections.length === 0" />
        
        <SettingsCard v-else-if="sortedConnections.length === 0">
             <div class="empty-state">
                <div class="empty-icon-wrap">
                    <i class="pi pi-shield"></i>
                </div>
                <p>No VPN connections configured</p>
                <small>Create a manual setup or import a configuration file to connect.</small>
            </div>
        </SettingsCard>

        <div v-else class="settings-card glass-panel" style="padding: 0;">
             <div class="settings-group-list">
                <div 
                    v-for="(conn, idx) in sortedConnections" 
                    :key="conn.uuid"
                    class="settings-item"
                    :class="{ 'selected': idx === selectedIndex }"
                    @click="toggleConnection(conn)"
                >
                    <!-- Vibrant dynamic colored glass icon -->
                    <div 
                        class="item-icon" 
                        :style="{ 
                            backgroundColor: getVpnTypeDetails(conn.type_name).bg,
                            border: '1px solid ' + getVpnTypeDetails(conn.type_name).border,
                            color: getVpnTypeDetails(conn.type_name).color
                        }"
                    >
                        <i :class="getVpnTypeDetails(conn.type_name).icon"></i>
                    </div>
                    <div class="item-details">
                        <span class="item-label">{{ conn.name }}</span>
                        <!-- Mini type badge -->
                        <span 
                            class="item-sublabel" 
                            :style="{ color: getVpnTypeDetails(conn.type_name).color }"
                        >
                            {{ getVpnTypeDetails(conn.type_name).label }}
                        </span>
                    </div>
                    
                    <div v-if="conn.uuid === transitioningUuid" class="connecting-label" style="justify-content: center; width: 44px;">
                        <i class="pi pi-spin pi-spinner" style="font-size: 1.1rem; color: var(--accent-color);"></i>
                    </div>
                    <div v-else class="switch-container">
                         <label class="switch click-passthrough">
                            <input type="checkbox" :checked="conn.active" readonly>
                            <span class="slider round"></span>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <ModalDialog 
        v-model="showAddModal" 
        :title="formData.mode === 'import' ? 'Import VPN Configuration' : 'Create VPN Profile'"
        @close="showAddModal = false"
    >
        <!-- Premium glass tabs at the top of the modal -->
        <div class="modal-tabs">
            <button 
                class="modal-tab" 
                :class="{ active: formData.mode === 'import' }"
                @click="formData.mode = 'import'; formData.type = 'openvpn'"
            >
                <i class="pi pi-file-import" style="font-size: 0.9rem;"></i>
                Import File
            </button>
            <button 
                class="modal-tab" 
                :class="{ active: formData.mode === 'manual' }"
                @click="formData.mode = 'manual'; formData.type = 'openvpn'"
            >
                <i class="pi pi-sliders-h" style="font-size: 0.9rem;"></i>
                Manual Setup
            </button>
        </div>

        <div class="form-container">
            <!-- COMMON TYPE SELECTOR -->
            <div class="form-group">
                <label>Giao thức VPN (Type)</label>
                <select v-model="formData.type" class="form-control">
                    <template v-if="formData.mode === 'import'">
                        <option value="openvpn">OpenVPN (.ovpn)</option>
                        <option value="wireguard">WireGuard (.conf, .wg)</option>
                    </template>
                    <template v-else>
                        <option value="openvpn">OpenVPN</option>
                        <option value="openconnect">Cisco AnyConnect (OpenConnect)</option>
                        <option value="l2tp">L2TP/IPsec</option>
                        <option value="pptp">PPTP</option>
                    </template>
                </select>
            </div>
            
            <!-- IMPORT CONFIGURATION MODE -->
            <template v-if="formData.mode === 'import'">
                <div class="form-group">
                    <label>Configuration File</label>
                    <div class="file-input-group">
                        <input type="text" v-model="formData.filePath" class="form-control" placeholder="Select file..." readonly @click="pickFile">
                        <button class="icon-btn" @click="pickFile">
                            <i class="pi pi-folder-open"></i>
                        </button>
                    </div>
                </div>

                <div class="form-group" v-if="formData.type === 'openvpn'">
                    <label>Username (Optional)</label>
                    <input type="text" v-model="formData.username" class="form-control" placeholder="Username">
                </div>

                <div class="form-group" v-if="formData.type === 'openvpn'">
                    <label>Password (Optional)</label>
                    <input type="password" v-model="formData.password" class="form-control" placeholder="Password">
                </div>
            </template>

            <!-- MANUAL CONFIGURATION MODE -->
            <template v-else>
                <div class="form-group">
                    <label>Tên kết nối (Connection Name)</label>
                    <input type="text" v-model="formData.name" class="form-control" placeholder="Office, Home, Corporate...">
                </div>

                <div class="form-group">
                    <label>Địa chỉ máy chủ (Gateway / Server)</label>
                    <input type="text" v-model="formData.gateway" class="form-control" placeholder="vpn.example.com or IP address">
                </div>

                <div class="form-group">
                    <label>Tên tài khoản (Username)</label>
                    <input type="text" v-model="formData.username" class="form-control" placeholder="Username (Optional)">
                </div>

                <div class="form-group">
                    <label>Mật khẩu (Password)</label>
                    <input type="password" v-model="formData.password" class="form-control" placeholder="Password (Optional)">
                </div>

                <!-- L2TP-specific PSK field -->
                <div class="form-group" v-if="formData.type === 'l2tp'">
                    <label>Khóa IPSec Pre-Shared Key (PSK)</label>
                    <input type="password" v-model="formData.psk" class="form-control" placeholder="IPSec Pre-Shared Key (Optional)">
                </div>
            </template>
        </div>

        <template #footer>
            <button class="btn-secondary" @click="showAddModal = false">Cancel</button>
            <button class="btn-primary" @click="saveConnection">
                {{ formData.mode === 'import' ? 'Import' : 'Create' }}
            </button>
        </template>
    </ModalDialog>
  </PageLayout>
</template>

<style scoped>
/* List Styles - Copied/Shared with BluetoothView */
.settings-group-list {
    display: flex;
    flex-direction: column;
}

.settings-item {
    display: flex;
    align-items: center;
    padding: 12px 18px;
    min-height: 52px;
    cursor: pointer;
    transition: background-color 0.2s ease;
    position: relative;
    border-bottom: 1px solid var(--card-border);
}

.settings-item:last-child {
    border-bottom: none;
}

.settings-item:hover {
    background-color: var(--item-hover-bg);
}

.item-icon {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 14px;
    font-size: 0.95rem;
    transition: all 0.25s ease;
}

.settings-item:hover .item-icon {
    transform: scale(1.05);
}

.item-details {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.item-label {
    font-size: 13.5px;
    font-weight: 500;
    color: var(--text-primary);
}

.item-sublabel {
    font-size: 10.5px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.connected-label {
    font-size: 12px;
    color: var(--text-secondary);
}

.connecting-label {
    font-size: 12px;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
}

.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 48px;
    color: var(--text-secondary);
    text-align: center;
}

.empty-icon-wrap {
    width: 52px;
    height: 52px;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    color: var(--text-secondary);
    opacity: 0.6;
    margin-bottom: 16px;
}

.click-passthrough {
    pointer-events: none;
}

.action-btn {
    background-color: var(--accent-color);
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    display: flex;
    align-items: center;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
}

.action-btn:hover {
    opacity: 0.9;
    transform: translateY(-1px);
}

/* Modal tab selectors */
.modal-tabs {
    display: flex;
    background-color: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--card-border);
    border-radius: 8px;
    padding: 3px;
    margin-bottom: 20px;
}

.modal-tab {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-tab:hover {
    color: var(--text-primary);
}

.modal-tab.active {
    background-color: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.05);
    color: var(--accent-color);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25);
}

.form-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 5px 0;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
}

.form-group label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-left: 2px;
}

.form-control {
    background-color: var(--bg-secondary);
    border: 1px solid transparent;
    border-radius: 8px;
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 13.5px;
    outline: none;
    transition: all 0.2s ease;
    width: 100%;
    box-sizing: border-box;
    appearance: none; 
}

.form-control::placeholder {
    color: var(--text-secondary);
    opacity: 0.5;
}

.form-control:focus {
    background-color: var(--card-bg);
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(var(--accent-rgb), 0.15);
}

/* Custom select styling arrow */
select.form-control {
    background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e5c197%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E");
    background-repeat: no-repeat;
    background-position: right 14px top 50%;
    background-size: 10px auto;
    padding-right: 32px;
}

.file-input-group {
    display: flex;
    gap: 8px;
}
.file-input-group .form-control {
    flex: 1;
    cursor: pointer;
}

.icon-btn {
    width: 40px;
    background-color: var(--bg-secondary);
    border: 1px solid transparent;
    border-radius: 8px;
    color: var(--text-primary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
}
.icon-btn:hover {
    background-color: var(--bg-hover);
    border-color: var(--accent-color);
}

.btn-primary, .btn-secondary {
    padding: 10px 18px;
    border-radius: 8px;
    font-size: 13.5px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
}

.btn-primary {
    background-color: var(--accent-color);
    color: white;
    font-weight: 600;
}
.btn-primary:hover {
    opacity: 0.95;
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(229, 193, 151, 0.25);
}

.btn-secondary {
    background-color: transparent;
    color: var(--text-secondary);
    border: 1px solid transparent;
}
.btn-secondary:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
}

.settings-item.selected {
    background-color: var(--item-hover-bg) !important;
    box-shadow: inset 3px 0 0 0 var(--accent-color) !important;
}
</style>
