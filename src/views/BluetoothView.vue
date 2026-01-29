<script setup lang="ts">
import { useBluetoothViewModel } from '../viewmodels/bluetooth.viewmodel';
import LoadingState from '@/components/LoadingState.vue';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';

const {
    isEnabled,
    sortedDevices,
    loading,
    connectingMac,
    toggleBluetooth,
    connect
} = useBluetoothViewModel();

const getDeviceIcon = (iconName?: string) => {
    if (!iconName) return 'pi pi-bluetooth';
    
    const icon = iconName.toLowerCase();

    switch (icon) {
        case 'audio-headphones':
            return 'pi pi-headphones';
        case 'audio':
            return 'pi pi-volume-up';
        case 'keyboard':
            return 'pi pi-microchip'; 
        case 'mouse':
            return 'pi pi-box'; 
        case 'phone':
            return 'pi pi-mobile';
        case 'computer':
            return 'pi pi-desktop';
        case 'display':
            return 'pi pi-desktop';
        case 'tablet':
            return 'pi pi-tablet';
        case 'network':
            return 'pi pi-wifi';
        case 'printer':
            return 'pi pi-print';
        case 'camera':
            return 'pi pi-camera';
        default:
            return 'pi pi-microchip';
    }
};
</script>

<template>
  <PageLayout>
    <template #title>Bluetooth</template>
    
    <template #actions>
        <div class="toggle-container">
            <span class="status-label">{{ isEnabled ? 'On' : 'Off' }}</span>
            <label class="switch">
                <input type="checkbox" v-model="isEnabled" @change="toggleBluetooth">
                <span class="slider round"></span>
            </label>
        </div>
    </template>

    <div v-if="isEnabled" class="device-list">
        <LoadingState v-if="loading" />
        
        <SettingsCard v-else-if="sortedDevices.length === 0">
             <div class="empty-state">
                <p>No devices found</p>
            </div>
        </SettingsCard>

        <div v-else class="settings-card glass-panel" style="padding: 0;">
             <!-- Using custom list style inside card -->
             <div class="settings-group-list">
                <div 
                    v-for="(dev, index) in sortedDevices" 
                    :key="dev.mac"
                    class="settings-item"
                    @click="connect(dev)"
                >
                    <div class="item-icon">
                        <i :class="getDeviceIcon(dev.icon)"></i>
                    </div>
                    <div class="item-details">
                        <span class="item-label">{{ dev.name || 'Unknown Device' }}</span>
                        <span class="item-sublabel">{{ dev.mac }}</span>
                    </div>
                    <div v-if="dev.connected" class="connected-label" style="color: var(--accent-color); font-size: 1.2rem;">
                        <i class="pi pi-check" style="font-weight: bold;"></i>
                    </div>
                    <div v-else-if="dev.mac === connectingMac" class="connecting-label">
                        <i class="pi pi-spin pi-spinner" style="font-size: 1rem; margin-right: 5px;"></i>
                        Connecting...
                    </div>
                    <div v-else class="action-label">
                        Connect
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <div v-else class="disabled-state">
        <i class="pi pi-bluetooth" style="font-size: 4rem; opacity: 0.2; margin-bottom: 1rem;"></i>
        <p>Bluetooth is turned off</p>
    </div>
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
