<script setup lang="ts">
import { useWifiViewModel } from '../viewmodels/wifi.viewmodel';
import WifiConfigModal from '@/components/wifi/WifiConfigModal.vue';
import LoadingState from '@/components/LoadingState.vue';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';
import loadingGif from '@/assets/loading-cat.gif';

const {
    isEnabled,
    networks,
    loading,
    connectingSsid,
    showConfigModal,
    savingConfig,
    selectedSsid,
    config,
    toggleWifi,
    connect,
    openConfig,
    closeConfig,
    saveConfig
} = useWifiViewModel();
</script>

<template>
  <PageLayout>
    <template #title>Wi-Fi</template>
    
    <template #actions>
        <div class="toggle-container">
            <span class="status-label">{{ isEnabled ? 'On' : 'Off' }}</span>
            <label class="switch">
                <input type="checkbox" v-model="isEnabled" @change="toggleWifi">
                <span class="slider round"></span>
            </label>
        </div>
    </template>

    <div v-if="isEnabled" class="network-list">
        <LoadingState v-if="loading" loading-text="Scanning for Wi-Fi network..." />
        
        <SettingsCard v-else-if="networks.length === 0">
             <div class="empty-state">
                <p>No networks found</p>
            </div>
        </SettingsCard>

        <div v-else class="settings-card glass-panel" style="padding: 0;">
            <div class="settings-group-list">
                <div 
                    v-for="net in networks" 
                    :key="net.ssid" 
                    class="settings-item"
                    :class="{ 
                        'disabled': connectingSsid !== null && connectingSsid !== net.ssid, 
                        'connecting': connectingSsid === net.ssid 
                    }"
                    @click="connect(net)"
                >
                    <div class="item-icon">
                        <i class="pi pi-wifi"></i>
                    </div>
                    <div class="item-details">
                        <div class="label-row">
                            <span class="item-label">{{ net.ssid }}</span>
                            <i v-if="net.security !== ''" class="pi pi-lock security-lock"></i>
                        </div>
                    </div>
                    
                    <!-- Status Indicators -->
                    <div v-if="connectingSsid === net.ssid" class="status-badge">
                        <img :src="loadingGif" style="width: 22px; height: 22px;" alt="Loading" />
                    </div>
                    <div v-else-if="net.active" class="connected-badge">
                        <i class="pi pi-check" style="color: var(--accent-color)"></i>
                    </div>
                    
                    <!-- Info Button -->
                    <a class="info-button" @click.stop="openConfig(net)">
                        <i class="pi pi-info-circle"></i>
                    </a>
                </div>
            </div>
        </div>
    </div>
    
    <div v-else class="disabled-state">
        <i class="pi pi-wifi" style="font-size: 4rem; opacity: 0.2; margin-bottom: 1rem;"></i>
        <p>Wi-Fi is turned off</p>
    </div>

    <!-- Config Modal Component -->
    <WifiConfigModal 
        :visible="showConfigModal"
        :ssid="selectedSsid"
        :config="config"
        :saving="savingConfig"
        @close="closeConfig"
        @save="saveConfig"
    />
  </PageLayout>
</template>

<style scoped>
.toggle-container {
    display: flex;
    align-items: center;
    gap: 10px;
}

.status-label {
    font-size: 14px;
    color: var(--text-secondary);
}

/* List Styles */
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
    background-color: var(--accent-color);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 14px;
    color: #ffffff;
}

.item-details {
    flex: 1;
    display: flex;
    flex-direction: column;
}

.item-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
}

.label-row {
    display: flex;
    align-items: center;
    gap: 6px;
}

.security-lock {
    font-size: 0.75rem;
    color: var(--text-secondary);
}

.empty-state, .disabled-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px;
    color: var(--text-secondary);
}

/* Info Button */
.info-button {
    margin-left: 10px;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    color: var(--text-secondary);
    transition: all 0.2s;
}

.info-button:hover {
    background-color: rgba(0,0,0,0.05);
    color: var(--accent-color);
}

/* Status Indicators */
.status-badge, .connected-badge {
    margin-left: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.settings-item.disabled {
    opacity: 0.5;
    pointer-events: none;
    cursor: default;
}
</style>
