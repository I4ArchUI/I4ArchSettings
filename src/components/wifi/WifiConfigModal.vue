<script setup lang="ts">
import type { WifiConfig } from '../../models/wifi.model';

interface Props {
    visible: boolean;
    ssid: string;
    config: WifiConfig; // This is reactive from parent
    saving: boolean;
}

interface Emits {
    (e: 'close'): void;
    (e: 'save'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const closeDialog = () => {
    emit('close');
};
</script>

<template>
    <Transition name="fade">
        <div v-if="visible" class="dialog-overlay" @click="closeDialog">
            <div class="dialog config-dialog" @click.stop>
                <div class="dialog-header">
                    <h3>Configure {{ ssid }}</h3>
                    <p class="dialog-subtitle">Network Settings</p>
                </div>
                
                <div class="dialog-body">
                    <div class="method-selector">
                        <label 
                            class="method-option" 
                            :class="{ active: config.method === 'auto' }"
                        >
                            <input type="radio" v-model="config.method" value="auto" class="hidden-radio">
                            <span class="method-name">Automatic (DHCP)</span>
                        </label>
                        <label 
                            class="method-option" 
                            :class="{ active: config.method === 'manual' }"
                        >
                            <input type="radio" v-model="config.method" value="manual" class="hidden-radio">
                            <span class="method-name">Manual (Static)</span>
                        </label>
                    </div>

                    <div v-if="config.method === 'manual'" class="manual-settings">
                        <div class="input-row">
                            <div class="form-group flex-2">
                                <label>IP Address</label>
                                <input type="text" v-model="config.ip_address" placeholder="192.168.1.50" class="flat-input">
                            </div>
                            <div class="form-group flex-1">
                                <label>Prefix / Mask</label>
                                <input type="number" v-model="config.prefix" placeholder="24" min="0" max="32" class="flat-input">
                            </div>
                        </div>
                        
                        <div class="form-group">
                            <label>Gateway</label>
                            <input type="text" v-model="config.gateway" placeholder="192.168.1.1" class="flat-input">
                        </div>
                    </div>
                    
                    <div v-else class="auto-info">
                       <p>IP address will be assigned automatically by the router.</p>
                    </div>

                    <!-- DNS Section (Always visible) -->
                    <div class="form-group" style="margin-top: 15px; border-top: 1px solid rgba(0,0,0,0.05); padding-top: 15px;">
                        <label>DNS Servers</label>
                        <input type="text" v-model="config.dns" placeholder="8.8.8.8, 1.1.1.1" class="flat-input">
                        <span class="input-hint">Leave empty to use router DNS</span>
                    </div>
                </div>

                <div class="dialog-actions">
                    <button class="btn-cancel" @click="closeDialog" :disabled="saving">Cancel</button>
                    <button class="btn-confirm" @click="emit('save')" :disabled="saving">
                        <i v-if="saving" class="pi pi-spin pi-spinner"></i>
                        {{ saving ? 'Applying...' : 'Apply Changes' }}
                    </button>
                </div>
            </div>
        </div>
    </Transition>
</template>

<style scoped>
/* Dialog Styles */
.dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.4);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.dialog {
    background: var(--content-bg, #1e1e1e);
    padding: 0;
    border-radius: 16px;
    width: 420px;
    max-width: 90vw;
    box-shadow: 0 10px 40px rgba(0,0,0,0.2);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.dialog-header {
    padding: 24px 24px 10px 24px;
}

.dialog h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
}

.dialog-subtitle {
    margin: 4px 0 0 0;
    font-size: 13px;
    color: var(--text-secondary);
}

.dialog-body {
    padding: 10px 24px 20px 24px;
}

/* Method Selector (Segmented Control) */
.method-selector {
    display: flex;
    background: rgba(0,0,0,0.05);
    padding: 4px;
    border-radius: 10px;
    margin-bottom: 20px;
}

:global(.dark) .method-selector {
    background: rgba(255,255,255,0.05);
}

.method-option {
    flex: 1;
    text-align: center;
    padding: 8px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 8px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: all 0.2s ease;
}

.method-option.active {
    background: var(--card-bg, #2a2a2a);
    color: var(--text-primary);
    box-shadow: 0 2px 5px rgba(0,0,0,0.05);
}

.hidden-radio {
    display: none;
}

/* Form Styles */
.form-group {
    margin-bottom: 16px;
}

.input-row {
    display: flex;
    gap: 12px;
}

.flex-1 { flex: 1; }
.flex-2 { flex: 2; }

.form-group label {
    display: block;
    margin-bottom: 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.flat-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid rgba(0,0,0,0.1);
    border-radius: 8px;
    background: var(--card-bg, #2a2a2a); 
    color: var(--text-primary);
    font-size: 14px;
    transition: border-color 0.2s, box-shadow 0.2s;
    outline: none;
    box-sizing: border-box;
}

:global(.dark) .flat-input {
    border: 1px solid rgba(255,255,255,0.1);
    background: rgba(255,255,255,0.03);
}

.flat-input:focus {
    border-color: var(--accent-color, #007aff);
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
}

.input-hint {
    display: block;
    margin-top: 6px;
    font-size: 11px;
    color: var(--text-secondary);
}

.auto-info {
    text-align: center;
    padding: 20px 0;
    color: var(--text-secondary);
    font-size: 13px;
}

/* Actions */
.dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px 24px;
    background: rgba(0,0,0,0.02);
    border-top: 1px solid rgba(0,0,0,0.05);
}

:global(.dark) .dialog-actions {
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
    color: var(--text-secondary);
    transition: background 0.2s;
}

.btn-cancel:hover {
    background: rgba(0,0,0,0.05);
    color: var(--text-primary);
}

.btn-confirm {
    padding: 8px 20px;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    font-weight: 600;
    background: var(--accent-color, #007aff);
    color: white;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: opacity 0.2s;
}

.btn-confirm:disabled {
    opacity: 0.7;
    cursor: not-allowed;
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
    from { opacity: 0; transform: translateY(20px) scale(0.95); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}

.manual-settings {
    animation: fadeIn 0.3s ease;
}
</style>
