<script setup lang="ts">
import { computed, ref, watch, onMounted, onUnmounted } from 'vue';
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
    connect,
    localName
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

/**
 * Computes coordinates, bubble sizes, and float animations for nearby devices.
 * Uses a radial distribution (polar coordinates) based on RSSI.
 */
const computedDevices = computed(() => {
    const list = sortedDevices.value;
    if (list.length === 0) return [];
    
    return list.map((dev, index) => {
        const angle = (2 * Math.PI * index) / list.length;
        
        const rssi = dev.rssi ?? -75; 
        const normalizedRssi = Math.max(-100, Math.min(-30, rssi));
        
        const percentage = (-normalizedRssi - 30) / 70; 
        const distance = 120 + percentage * 150; 

        const x = 350 + distance * Math.cos(angle);
        const y = 350 + distance * Math.sin(angle);
        const size = 115 - percentage * 35;
        
        return {
            ...dev,
            x,
            y,
            size,
            distance,
            angle,
            rssiLabel: rssi,
            floatClass: `float-anim-${(index % 3) + 1}`
        };
    });
});

const triggerToggle = () => {
    isEnabled.value = !isEnabled.value;
    toggleBluetooth();
};

onMounted(() => {
    window.addEventListener('shortcut-toggle', triggerToggle);
    window.addEventListener('keydown', handleListKeyDown);
});

onUnmounted(() => {
    window.removeEventListener('shortcut-toggle', triggerToggle);
    window.removeEventListener('keydown', handleListKeyDown);
});

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

    if (!isEnabled.value || sortedDevices.value.length === 0) return;

    if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {
        selectedIndex.value = (selectedIndex.value + 1) % sortedDevices.value.length;
        e.preventDefault();
    } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
        selectedIndex.value = (selectedIndex.value - 1 + sortedDevices.value.length) % sortedDevices.value.length;
        e.preventDefault();
    } else if (e.key === 'Enter') {
        const dev = sortedDevices.value[selectedIndex.value];
        if (dev) {
            connect(dev);
        }
        e.preventDefault();
    }
};

watch(sortedDevices, (newVal) => {
    if (selectedIndex.value >= newVal.length) {
        selectedIndex.value = Math.max(0, newVal.length - 1);
    }
});
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
            <span class="kbd-hint"><kbd>Alt</kbd>+<kbd>S</kbd></span>
        </div>
    </template>

    <div v-if="isEnabled" class="bluetooth-radar-panel">
        <LoadingState v-if="loading && sortedDevices.length === 0" loading-text="Searching for devices..." />
        
        <SettingsCard v-else-if="sortedDevices.length === 0">
             <div class="empty-state">
                <div class="radar-scan-circle">
                    <i class="pi pi-bluetooth animate-pulse"></i>
                </div>
                <p>No devices found</p>
                <small>Make sure your Bluetooth devices are in pairing mode.</small>
             </div>
        </SettingsCard>

        <!-- HIGH TECH RADAR BOARD -->
        <div v-else class="radar-container">
            <div class="radar-board">
                <!-- SVG BACKGROUND NETWORK AND CONCENTRIC RINGS -->
                <svg class="radar-svg" viewBox="0 0 700 700">
                    <!-- Connection lines -->
                    <line 
                        v-for="dev in computedDevices" 
                        :key="'line-' + dev.mac"
                        x1="350" 
                        y1="350" 
                        :x2="dev.x" 
                        :y2="dev.y" 
                        class="connection-line"
                        :class="{ 
                            connected: dev.connected, 
                            connecting: dev.mac === connectingMac,
                            unconnected: !dev.connected && dev.mac !== connectingMac 
                        }"
                    />
                </svg>
                
                <!-- MY DEVICE (CENTER OBJECT) -->
                <div class="center-device">
                    <div class="center-ring-pulse"></div>
                    <div class="center-bubble glass-panel">
                        <i class="pi pi-bluetooth center-icon"></i>
                        <span class="center-label">{{ localName }}</span>
                        <span class="center-sublabel">My Device</span>
                    </div>
                </div>
                
                <!-- FLOAT DEVICES (ORBIT NODES) -->
                <div 
                    v-for="(dev, idx) in computedDevices" 
                    :key="dev.mac"
                    class="device-bubble-wrap"
                    :class="[dev.floatClass, { 
                        'is-connected': dev.connected, 
                        'is-connecting': dev.mac === connectingMac,
                        'selected': idx === selectedIndex
                    }]"
                    :style="{ 
                        left: dev.x + 'px', 
                        top: dev.y + 'px',
                        width: dev.size + 'px',
                        height: dev.size + 'px'
                    }"
                    @click="connect(dev)"
                >
                    <div class="device-bubble">
                        <!-- BOTH DEVICE NAME AND ICON RENDERED INSIDE BUBBLE -->
                        <div class="bubble-inner-content">
                            <i :class="getDeviceIcon(dev.icon)" class="device-icon"></i>
                            <span class="device-name-label">{{ dev.name || 'Unknown' }}</span>
                        </div>
                        
                        <!-- Connection status indicator inside bubble -->
                        <span v-if="dev.connected" class="status-indicator connected">
                            <i class="pi pi-check"></i>
                        </span>
                        <span v-else-if="dev.mac === connectingMac" class="status-indicator connecting">
                            <i class="pi pi-spin pi-spinner"></i>
                        </span>
                    </div>
                    
                    <!-- PREMIUM GLASS TOOLTIP ON HOVER -->
                    <div class="device-tooltip glass-panel">
                        <div class="tooltip-title">{{ dev.name || 'Unknown Device' }}</div>
                        <div class="tooltip-row">
                            <span class="t-lbl">MAC:</span>
                            <span class="t-val">{{ dev.mac }}</span>
                        </div>
                        <div class="tooltip-row" v-if="dev.rssiLabel">
                            <span class="t-lbl">Signal:</span>
                            <span class="t-val" :style="{ color: dev.connected ? 'var(--accent-color)' : '#29b6f6' }">
                                {{ dev.rssiLabel }} dBm
                            </span>
                        </div>
                        <div class="tooltip-row">
                            <span class="t-lbl">Status:</span>
                            <span class="t-val" :style="{ color: dev.connected ? 'var(--accent-color)' : '#888' }">
                                {{ dev.connected ? 'Connected' : (dev.mac === connectingMac ? 'Connecting...' : 'Disconnected') }}
                            </span>
                        </div>
                        <div class="tooltip-footer" v-if="!dev.connected && dev.mac !== connectingMac">
                            Click to connect
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <div v-else class="disabled-state glass-panel">
        <div class="disabled-icon-wrap">
            <i class="pi pi-bluetooth"></i>
        </div>
        <p>Bluetooth is turned off</p>
        <small>Toggle the switch above to start discovery</small>
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
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
}

/* Radar Container & Board */
.bluetooth-radar-panel {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px 0;
}

.radar-container {
    width: 700px;
    height: 700px;
    position: relative;
    border-radius: 20px;
    padding: 0;
    overflow: hidden;
    /* background: rgba(255, 255, 255, 0.01) !important; */
}

.radar-board {
    width: 100%;
    height: 100%;
    position: relative;
}

/* SVG Radar Background */
.radar-svg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 1;
}

.radar-ring {
    fill: none;
    stroke: rgba(255, 255, 255, 0.03);
    stroke-width: 1px;
}

.radar-label {
    font-size: 10px;
    fill: var(--text-secondary);
    opacity: 0.35;
    font-weight: 500;
}

/* Radar range lines */
.connection-line {
    fill: none;
    transition: stroke 0.3s ease, stroke-width 0.3s ease;
}

/* Connected Line - Pulsing Glowing Yellow */
@keyframes line-pulse {
    to {
        stroke-dashoffset: -20;
    }
}
.connection-line.connected {
    stroke: var(--accent-color);
    stroke-width: 2px;
    stroke-dasharray: 6 4;
    animation: line-pulse 1.2s linear infinite;
    filter: drop-shadow(0 0 3px rgba(229, 193, 151, 0.6));
    opacity: 0.8;
}

/* Connecting Line - Rapid Yellow Pulse */
.connection-line.connecting {
    stroke: var(--accent-color);
    stroke-width: 2px;
    stroke-dasharray: 4 4;
    animation: line-pulse 0.6s linear infinite;
    opacity: 0.8;
}

/* Unconnected Line - Thin Dashed Blue */
.connection-line.unconnected {
    stroke: rgba(41, 182, 246, 0.25);
    stroke-width: 1px;
    stroke-dasharray: 4 6;
    opacity: 0.6;
}

/* Center Device (My Device) */
.center-device {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 5;
}

.center-bubble {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: rgba(30, 30, 30, 0.85) !important;
    border: 1px solid rgba(255, 255, 255, 0.12) !important;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4) !important;
    padding: 10px;
    text-align: center;
    cursor: default;
}

.center-icon {
    font-size: 1.8rem;
    color: var(--accent-color);
    margin-bottom: 6px;
    filter: drop-shadow(0 0 6px rgba(229, 193, 151, 0.4));
}

.center-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    width: 90px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.center-sublabel {
    font-size: 9px;
    color: var(--text-secondary);
    opacity: 0.6;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-top: 2px;
}

/* Pulse animation around center */
@keyframes pulse-ring {
    0% {
        transform: translate(-50%, -50%) scale(0.95);
        opacity: 0.6;
    }
    50% {
        opacity: 0.2;
    }
    100% {
        transform: translate(-50%, -50%) scale(1.3);
        opacity: 0;
    }
}
.center-ring-pulse {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 130px;
    height: 130px;
    border-radius: 50%;
    border: 1.5px solid rgba(229, 193, 151, 0.2);
    animation: pulse-ring 3s cubic-bezier(0.215, 0.61, 0.355, 1) infinite;
    pointer-events: none;
}

/* Device Bubble Orbit Nodes */
.device-bubble-wrap {
    position: absolute;
    z-index: 4;
    cursor: pointer;
    transform: translate(-50%, -50%);
}

.device-bubble {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    backdrop-filter: blur(12px);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* Inner content alignment */
.bubble-inner-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 85%;
    height: 85%;
    gap: 6px;
    text-align: center;
}

.device-name-label {
    font-size: 9px;
    font-weight: 600;
    color: var(--text-primary);
    width: 90%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    opacity: 0.9;
}

.device-icon {
    font-size: 1.2rem;
    transition: all 0.3s ease;
}

.device-bubble-wrap:hover .device-icon {
    transform: scale(1.1);
}

/* Color Coding Rules - Not too transparent (opacity 0.85) */
/* CONNECTED BUBBLE - YELLOW (AMBER/GOLD) */
.device-bubble-wrap.is-connected .device-bubble {
    background: rgba(30, 26, 22, 0.85);
    border: 1.5px solid rgba(229, 193, 151, 0.55);
    color: var(--accent-color);
    box-shadow: 0 0 15px rgba(229, 193, 151, 0.25), inset 0 0 8px rgba(229, 193, 151, 0.15);
}

.device-bubble-wrap.is-connected:hover .device-bubble {
    background: rgba(34, 29, 24, 0.9);
    border-color: var(--accent-color);
    box-shadow: 0 0 25px rgba(229, 193, 151, 0.45);
    transform: scale(1.05);
}

/* CONNECTING BUBBLE - FLASHING YELLOW */
@keyframes connect-glow {
    0% { box-shadow: 0 0 10px rgba(229, 193, 151, 0.25); border-color: rgba(229, 193, 151, 0.4); }
    50% { box-shadow: 0 0 25px rgba(229, 193, 151, 0.55); border-color: var(--accent-color); }
    100% { box-shadow: 0 0 10px rgba(229, 193, 151, 0.25); border-color: rgba(229, 193, 151, 0.4); }
}
.device-bubble-wrap.is-connecting .device-bubble {
    background: rgba(30, 26, 22, 0.85);
    border: 1.5px solid rgba(229, 193, 151, 0.4);
    color: var(--accent-color);
    animation: connect-glow 1.5s infinite;
}

/* UNCONNECTED BUBBLE - BLUE */
.device-bubble-wrap:not(.is-connected):not(.is-connecting) .device-bubble {
    background: rgba(20, 24, 30, 0.85);
    border: 1.5px solid rgba(41, 182, 246, 0.45);
    color: #29b6f6;
    box-shadow: 0 0 12px rgba(41, 182, 246, 0.15);
}

.device-bubble-wrap:not(.is-connected):not(.is-connecting):hover .device-bubble {
    background: rgba(23, 28, 36, 0.9);
    border-color: #29b6f6;
    box-shadow: 0 0 22px rgba(41, 182, 246, 0.35);
    color: #4fc3f7;
    transform: scale(1.05);
}

/* Status Indicator Mini-Bages */
.status-indicator {
    position: absolute;
    bottom: -2px;
    right: -2px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    box-shadow: 0 2px 5px rgba(0,0,0,0.3);
}

.status-indicator.connected {
    background-color: var(--accent-color);
    color: #1e1e1e;
}

.status-indicator.connecting {
    background-color: #ffb300;
    color: #1e1e1e;
}

/* Premium Frosted Tooltip on Hover */
.device-tooltip {
    position: absolute;
    top: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%) scale(0.95);
    opacity: 0;
    pointer-events: none;
    z-index: 10;
    width: 160px;
    padding: 10px 12px !important;
    border-radius: 10px !important;
    background: rgba(26, 26, 26, 0.88) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    box-shadow: 0 10px 25px rgba(0,0,0,0.5) !important;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: left;
}

.device-bubble-wrap:hover .device-tooltip {
    opacity: 1;
    transform: translateX(-50%) scale(1);
}

.tooltip-title {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-primary);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 4px;
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.tooltip-row {
    display: flex;
    justify-content: space-between;
    font-size: 9.5px;
}

.t-lbl {
    color: var(--text-secondary);
    opacity: 0.65;
}

.t-val {
    color: var(--text-primary);
    font-weight: 500;
    max-width: 100px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.tooltip-footer {
    font-size: 8.5px;
    color: var(--accent-color);
    font-weight: 600;
    text-align: center;
    border-top: 1px solid rgba(255,255,255,0.05);
    padding-top: 4px;
    margin-top: 2px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

/* Floating Animations for Organic Nodes */
@keyframes float-1 {
    0% { transform: translate(-50%, -50%) translate(0, 0px); }
    50% { transform: translate(-50%, -50%) translate(-3px, -8px); }
    100% { transform: translate(-50%, -50%) translate(0, 0px); }
}

@keyframes float-2 {
    0% { transform: translate(-50%, -50%) translate(0, 0px); }
    50% { transform: translate(-50%, -50%) translate(4px, -6px); }
    100% { transform: translate(-50%, -50%) translate(0, 0px); }
}

@keyframes float-3 {
    0% { transform: translate(-50%, -50%) translate(0, 0px); }
    50% { transform: translate(-50%, -50%) translate(-4px, -4px); }
    100% { transform: translate(-50%, -50%) translate(0, 0px); }
}

.float-anim-1 {
    animation: float-1 4.5s ease-in-out infinite;
}

.float-anim-2 {
    animation: float-2 5.5s ease-in-out infinite;
}

.float-anim-3 {
    animation: float-3 5s ease-in-out infinite;
}

/* Scanner Circle during scan & Empty State */
.radar-scan-circle {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background: rgba(41, 182, 246, 0.1);
    border: 1px solid rgba(41, 182, 246, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.8rem;
    color: #29b6f6;
    margin-bottom: 16px;
}

.animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: .4; }
}

.empty-state, .disabled-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px;
    color: var(--text-secondary);
    text-align: center;
}

.disabled-state {
    padding: 100px 40px;
}

.disabled-icon-wrap {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2.2rem;
    color: var(--text-secondary);
    opacity: 0.25;
    margin-bottom: 20px;
}

/* Selected state for keyboard navigation */
.device-bubble-wrap.selected .device-bubble {
    outline: 2px solid var(--accent-color) !important;
    outline-offset: 4px;
    box-shadow: 0 0 25px rgba(229, 193, 151, 0.65) !important;
    transform: scale(1.08);
}

.device-bubble-wrap.selected .device-tooltip {
    opacity: 1;
    pointer-events: auto;
    transform: translateX(-50%) scale(1);
}
</style>
