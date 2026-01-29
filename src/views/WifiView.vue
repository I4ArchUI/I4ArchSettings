<script setup lang="ts">
import { useWifiViewModel } from '../viewmodels/wifi.viewmodel';
import WifiConfigModal from '@/components/wifi/WifiConfigModal.vue';
import LoadingState from '@/components/LoadingState.vue';

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
  <div class="wifi-view">
    <div class="header">
        <h1 class="page-title">Wi-Fi</h1>
        <div class="toggle-container">
            <span class="status-label">{{ isEnabled ? 'On' : 'Off' }}</span>
            <label class="switch">
                <input type="checkbox" v-model="isEnabled" @change="toggleWifi">
                <span class="slider round"></span>
            </label>
        </div>
    </div>

    <div v-if="isEnabled" class="network-list">
        <LoadingState v-if="loading" />
        
        <div v-else-if="networks.length === 0" class="empty-state">
            <p>No networks found</p>
        </div>

        <div v-else class="settings-group">
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
                    <i class="pi pi-spin pi-spinner" style="color: var(--accent-color)"></i>
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
  </div>
</template>

<style scoped>
.wifi-view {
    padding: 0 40px 40px 40px;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
}

.header {
    padding: 20px 0 16px 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.page-title {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
    color: var(--text-primary);
}

.toggle-container {
    display: flex;
    align-items: center;
    gap: 10px;
}

.status-label {
    font-size: 14px;
    color: var(--text-secondary);
}

/* Switch Styles */
.switch {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 24px;
}

.switch input { 
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  -webkit-transition: .4s;
  transition: .4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 20px;
  width: 20px;
  left: 2px;
  bottom: 2px;
  background-color: white;
  -webkit-transition: .4s;
  transition: .4s;
}

input:checked + .slider {
  background-color: var(--accent-color, #007aff);
}

input:focus + .slider {
  box-shadow: 0 0 1px var(--accent-color, #007aff);
}

input:checked + .slider:before {
  -webkit-transform: translateX(16px);
  -ms-transform: translateX(16px);
  transform: translateX(16px);
}

.slider.round {
  border-radius: 34px;
}

.slider.round:before {
  border-radius: 50%;
}

/* List Styles */
.settings-group {
    background-color: var(--card-bg);
    border-radius: 10px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
    border: 1px solid var(--separator-color); 
    border: 1px solid rgba(0,0,0,0.05);
    overflow: hidden;
}

.settings-item {
    display: flex;
    align-items: center;
    padding: 10px 16px;
    min-height: 48px;
    cursor: pointer;
    background-color: var(--card-bg);
    transition: background-color 0.1s;
    position: relative;
    border-bottom: 1px solid var(--separator-color);
}

.settings-item:last-child {
    border-bottom: none;
}

.settings-item:hover {
    background-color: var(--item-hover-bg, rgba(0,0,0,0.03));
}

.item-icon {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background-color: var(--accent-color, #007aff);
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

.signal-indicator {
    margin-left: 10px;
    color: var(--text-secondary);
    font-family: monospace;
    font-size: 12px;
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
