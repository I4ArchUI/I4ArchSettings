<script setup lang="ts">
import { useBluetoothViewModel } from '../viewmodels/bluetooth.viewmodel';
import LoadingState from '@/components/LoadingState.vue';

const {
    isEnabled,
    devices,
    loading,
    toggleBluetooth,
    connect
} = useBluetoothViewModel();
</script>

<template>
  <div class="bluetooth-view">
    <div class="header">
        <h1 class="page-title">Bluetooth</h1>
        <div class="toggle-container">
            <span class="status-label">{{ isEnabled ? 'On' : 'Off' }}</span>
            <label class="switch">
                <input type="checkbox" v-model="isEnabled" @change="toggleBluetooth">
                <span class="slider round"></span>
            </label>
        </div>
    </div>

    <div v-if="isEnabled" class="device-list">
        <LoadingState v-if="loading" />
        
        <div v-else-if="devices.length === 0" class="empty-state">
            <p>No devices found</p>
        </div>

        <div v-else class="settings-group">
            <div 
                v-for="(dev, index) in devices" 
                :key="index" 
                class="settings-item"
                @click="connect(dev)"
            >
                <div class="item-icon">
                    <i class="pi pi-bluetooth"></i>
                </div>
                <div class="item-details">
                    <span class="item-label">{{ dev.name || 'Unknown Device' }}</span>
                    <span class="item-sublabel">{{ dev.mac }}</span>
                </div>
                <div v-if="dev.connected" class="connected-label">
                    Connected
                </div>
                <div v-else class="action-label">
                    Connect
                </div>
            </div>
        </div>
    </div>
    
    <div v-else class="disabled-state">
        <i class="pi pi-bluetooth" style="font-size: 4rem; opacity: 0.2; margin-bottom: 1rem;"></i>
        <p>Bluetooth is turned off</p>
    </div>
  </div>
</template>

<style scoped>
.bluetooth-view {
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

/* List Styles */
.settings-group {
    background-color: var(--card-bg);
    border-radius: 10px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
    border: 1px solid var(--card-border); 
    /* removed duplicate border definitions */
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

.action-label {
    font-size: 12px;
    color: var(--accent-color);
    display: none;
}

.settings-item:hover .action-label {
    display: block;
}

.empty-state, .disabled-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px;
    color: var(--text-secondary);
}
</style>
