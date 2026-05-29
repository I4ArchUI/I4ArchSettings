<script setup lang="ts">
import { computed } from 'vue';
import type { WifiNetwork, WifiConfig } from '../../models/wifi.model';

interface Props {
    visible: boolean;
    network: WifiNetwork | null;
    config: WifiConfig | null;
}

interface Emits {
    (e: 'close'): void;
    (e: 'configure'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const closeDialog = () => {
    emit('close');
};

const handleConfigure = () => {
    emit('configure');
};

// Calculate subnet mask from CIDR prefix
const subnetMask = computed(() => {
    if (!props.config || props.config.prefix === undefined) return '255.255.255.0';
    const prefix = props.config.prefix;
    let mask = [];
    for (let i = 0; i < 4; i++) {
        let n = Math.min(Math.max(prefix - i * 8, 0), 8);
        mask.push(256 - Math.pow(2, 8 - n));
    }
    return `${mask.join('.')}`;
});

const securityLabel = computed(() => {
    if (!props.network || !props.network.security) return 'Open (Unsecured)';
    return props.network.security;
});

const signalLabel = computed(() => {
    if (!props.network) return 'Unknown';
    const signal = props.network.signal !== undefined ? props.network.signal : 0;
    if (signal > 80) return `Strong (${signal}%)`;
    if (signal > 50) return `Medium (${signal}%)`;
    if (signal > 20) return `Weak (${signal}%)`;
    return `Very Weak (${signal}%)`;
});
</script>

<template>
    <Transition name="fade">
        <div v-if="visible" class="dialog-overlay" @click="closeDialog">
            <div class="dialog info-dialog" @click.stop>
                <div class="dialog-header">
                    <h3>{{ network?.ssid }}</h3>
                    <div class="status-indicator-wrapper">
                        <span class="status-dot" :class="{ 'active': network?.active }"></span>
                        <span class="status-text" :class="{ 'connected': network?.active }">
                            {{ network?.active ? 'Connected' : 'Saved' }}
                        </span>
                    </div>
                </div>
                
                <div class="dialog-body">
                    <div class="info-section">
                        <h4 class="section-title">Wireless Connection</h4>
                        <div class="info-grid">
                            <div class="info-item">
                                <span class="info-label">Security</span>
                                <span class="info-value">{{ securityLabel }}</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">Signal Strength</span>
                                <span class="info-value">{{ signalLabel }}</span>
                            </div>
                            <div v-if="config?.interface" class="info-item">
                                <span class="info-label">Interface</span>
                                <span class="info-value">{{ config.interface }}</span>
                            </div>
                            <div v-if="config?.mac_address" class="info-item">
                                <span class="info-label">Device MAC</span>
                                <span class="info-value">{{ config.mac_address }}</span>
                            </div>
                            <div v-if="config?.bssid" class="info-item">
                                <span class="info-label">AP BSSID</span>
                                <span class="info-value">{{ config.bssid }}</span>
                            </div>
                            <div v-if="config?.frequency" class="info-item">
                                <span class="info-label">Frequency</span>
                                <span class="info-value">{{ config.frequency }}</span>
                            </div>
                            <div v-if="config?.speed" class="info-item">
                                <span class="info-label">Speed</span>
                                <span class="info-value">{{ config.speed }}</span>
                            </div>
                        </div>
                    </div>

                    <div class="info-section" style="margin-top: 20px;">
                        <h4 class="section-title">IPv4 Configuration</h4>
                        <div class="info-grid">
                            <div class="info-item">
                                <span class="info-label">IP Assignment</span>
                                <span class="info-value capitalize">{{ config?.method === 'manual' ? 'Static (Manual)' : 'DHCP (Automatic)' }}</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">IPv4 Address</span>
                                <span class="info-value">{{ config?.ip_address || 'Not Assigned' }}</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">Subnet Mask</span>
                                <span class="info-value">{{ config?.ip_address ? `${subnetMask} (Prefix: /${config.prefix})` : 'Not Available' }}</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">Default Gateway</span>
                                <span class="info-value">{{ config?.gateway || 'Not Available' }}</span>
                            </div>
                            <div class="info-item full-width">
                                <span class="info-label">DNS Servers</span>
                                <span class="info-value">{{ config?.dns || 'Automatic (Router Default)' }}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="dialog-actions">
                    <button class="btn-cancel" @click="closeDialog">Close</button>
                    <button class="btn-confirm" @click="handleConfigure">
                        <i class="pi pi-cog"></i> Configure IP
                    </button>
                </div>
            </div>
        </div>
    </Transition>
</template>

<style scoped>
.dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.5);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1050;
}

.dialog {
    background: rgba(26, 27, 30, 0.75);
    border: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(20px);
    padding: 0;
    border-radius: 16px;
    width: 440px;
    max-width: 90vw;
    box-shadow: 0 16px 40px rgba(0,0,0,0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.dialog-header {
    padding: 24px 24px 12px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.dialog h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
}

.status-indicator-wrapper {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.04);
    padding: 4px 10px;
    border-radius: 20px;
    border: 1px solid rgba(255,255,255,0.04);
}

.status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #9e9e9e;
}

.status-dot.active {
    background: var(--accent-color, #e5c197);
    box-shadow: 0 0 8px var(--accent-color, #e5c197);
    animation: pulse 2s infinite;
}

.status-text {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary, #9e9e9e);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.status-text.connected {
    color: var(--accent-color, #e5c197);
}

.dialog-body {
    padding: 12px 24px 24px 24px;
    max-height: 480px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 20px;
}

/* Custom Scrollbar */
.dialog-body::-webkit-scrollbar {
    width: 6px;
}
.dialog-body::-webkit-scrollbar-track {
    background: transparent;
}
.dialog-body::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
}
.dialog-body::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.2);
}

.info-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.section-title {
    margin: 0;
    font-size: 11px;
    font-weight: 700;
    color: var(--accent-color, #e5c197);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    padding-bottom: 6px;
}

/* Info Grid Layout */
.info-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
}

.info-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.info-item.full-width {
    grid-column: span 2;
}

.info-label {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary, #9e9e9e);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.info-value {
    font-size: 13px;
    color: var(--text-primary, #ffffff);
    font-family: 'Outfit', sans-serif;
    word-break: break-all;
}

.capitalize {
    text-transform: capitalize;
}

.dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px 24px;
    background: rgba(255,255,255,0.02);
    border-top: 1px solid rgba(255,255,255,0.05);
}

.btn-cancel {
    padding: 8px 16px;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    font-weight: 500;
    background: transparent;
    color: var(--text-secondary, #9e9e9e);
    transition: all 0.2s;
}

.btn-cancel:hover {
    background: rgba(255,255,255,0.05);
    color: var(--text-primary, #ffffff);
}

.btn-confirm {
    padding: 8px 20px;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    font-weight: 600;
    background: var(--accent-color, #e5c197);
    color: #121214;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: opacity 0.2s, transform 0.1s;
}

.btn-confirm:hover {
    opacity: 0.9;
}

.btn-confirm:active {
    transform: scale(0.98);
}

/* Animations */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@keyframes slideUp {
    from { opacity: 0; transform: translateY(15px) scale(0.97); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}

@keyframes pulse {
    0% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.2); opacity: 0.7; }
    100% { transform: scale(1); opacity: 1; }
}
</style>
