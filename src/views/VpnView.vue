<script setup lang="ts">
import { useVpnViewModel } from '../viewmodels/vpn.viewmodel';
import LoadingState from '@/components/LoadingState.vue';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';
import ModalDialog from '@/components/common/ModalDialog.vue';

const {
    sortedConnections,
    loading,
    connectingUuid,
    toggleConnection,
    showAddModal,
    formData,
    openAddModal,
    pickFile,
    saveConnection
} = useVpnViewModel();

</script>

<template>
  <PageLayout>
    <template #title>VPN</template>

    <template #actions>
        <button class="action-btn" @click="openAddModal">
            <i class="pi pi-plus" style="margin-right: 6px;"></i>
            Import VPN
        </button>
    </template>
    
    <div class="vpn-list">
        <LoadingState v-if="loading && sortedConnections.length === 0" />
        
        <SettingsCard v-else-if="sortedConnections.length === 0">
             <div class="empty-state">
                <p>No VPN connections configured</p>
                <small>Use Network Manager to add VPN connections</small>
            </div>
        </SettingsCard>

        <div v-else class="settings-card glass-panel" style="padding: 0;">
             <!-- Using custom list style inside card -->
             <div class="settings-group-list">
                <div 
                    v-for="conn in sortedConnections" 
                    :key="conn.uuid"
                    class="settings-item"
                    @click="toggleConnection(conn)"
                >
                    <div class="item-icon">
                        <i class="pi pi-shield"></i>
                    </div>
                    <div class="item-details">
                        <span class="item-label">{{ conn.name }}</span>
                        <span class="item-sublabel">{{ conn.type_name }}</span>
                    </div>
                    
                    <div v-if="conn.active" class="connected-label" style="color: var(--accent-color); font-size: 1.2rem;">
                         <i class="pi pi-check" style="font-weight: bold;"></i>
                    </div>
                    <div v-else-if="conn.uuid === connectingUuid" class="connecting-label">
                        <i class="pi pi-spin pi-spinner" style="font-size: 1rem; margin-right: 5px;"></i>
                        Connecting...
                    </div>
                    <div v-else class="switch-container">
                         <!-- Visual toggle for consistency, though list click works too -->
                         <div class="action-label" style="display:none">Connect</div>
                         <!-- Add a toggle switch that reflects state -->
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
        title="Import VPN"
        @close="showAddModal = false"
    >
        <div class="form-container">
            <div class="form-group">
                <label>VPN Type</label>
                <select v-model="formData.type" class="form-control">
                    <option value="openvpn">OpenVPN</option>
                    <option value="wireguard">WireGuard</option>
                </select>
            </div>
            
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
        </div>

        <template #footer>
            <button class="btn-secondary" @click="showAddModal = false">Cancel</button>
            <button class="btn-primary" @click="saveConnection">Import</button>
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
    padding: 10px 16px;
    min-height: 48px;
    cursor: pointer;
    transition: background-color 0.1s;
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
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background-color: var(--bg-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 14px;
    color: var(--text-secondary);
}

.item-details {
    flex: 1;
    display: flex;
    flex-direction: column;
}

.item-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
}

.item-sublabel {
    font-size: 11px;
    color: var(--text-secondary);
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
    padding: 40px;
    color: var(--text-secondary);
    text-align: center;
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

.form-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 5px 0;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.form-group label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    margin-left: 2px;
}

.form-control {
    background-color: var(--bg-secondary);
    border: 1px solid transparent;
    border-radius: 8px;
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 14px;
    outline: none;
    transition: all 0.2s ease;
    width: 100%;
    box-sizing: border-box;
    /* Clean up default styling */
    appearance: none; 
}

.form-control::placeholder {
    color: var(--text-secondary);
    opacity: 0.7;
}

.form-control:focus {
    background-color: var(--card-bg);
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(var(--accent-rgb), 0.15);
}

/* Custom select styling arrow */
select.form-control {
    background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23999%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E");
    background-repeat: no-repeat;
    background-position: right 12px top 50%;
    background-size: 10px auto;
    padding-right: 30px;
}

.file-input-group {
    display: flex;
    gap: 10px;
}
.file-input-group .form-control {
    flex: 1;
    cursor: pointer;
    background-color: var(--bg-input, #2a2a2a);
}

.icon-btn {
    width: 42px;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color, #444);
    border-radius: 8px;
    color: var(--text-primary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
}
.icon-btn:hover {
    background-color: var(--bg-hover);
    border-color: var(--accent-color);
}

.btn-primary, .btn-secondary {
    padding: 10px 20px;
    border-radius: 8px;
    font-size: 14px;
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
    opacity: 0.9;
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(var(--accent-color-rgb), 0.3);
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
</style>
