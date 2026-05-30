<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from 'vue';

interface Props {
    visible: boolean;
    ssid: string;
    saving: boolean;
    errorMsg?: string | null;
    security?: string;
}

interface Emits {
    (e: 'close'): void;
    (e: 'connect', password: string, username?: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const username = ref('');
const password = ref('');
const showPassword = ref(false);

const isEnterprise = computed(() => {
    if (!props.security) return false;
    const sec = props.security.toLowerCase();
    return sec.includes('enterprise') || sec.includes('eap') || sec.includes('802.1x');
});

// Reset values when visibility changes
watch(() => props.visible, (newVal) => {
    if (newVal) {
        username.value = '';
        password.value = '';
        showPassword.value = false;
    }
});

const closeDialog = () => {
    emit('close');
};

const onKeydown = (e: KeyboardEvent) => {
    if (e.key === 'Escape' && props.visible && !props.saving) {
        closeDialog();
    }
};

onMounted(() => {
    document.addEventListener('keydown', onKeydown);
});

onUnmounted(() => {
    document.removeEventListener('keydown', onKeydown);
});

const handleConnect = () => {
    if (password.value.trim().length > 0) {
        if (isEnterprise.value) {
            if (username.value.trim().length > 0) {
                emit('connect', password.value, username.value);
            }
        } else {
            emit('connect', password.value);
        }
    }
};

const toggleShowPassword = () => {
    showPassword.value = !showPassword.value;
};
</script>

<template>
    <Transition name="fade">
        <div v-if="visible" class="dialog-overlay" @click="closeDialog">
            <div class="dialog password-dialog" @click.stop>
                <div class="dialog-header">
                    <h3>Connect to {{ ssid }}</h3>
                    <p class="dialog-subtitle">
                        {{ isEnterprise ? 'This enterprise network requires credentials.' : 'This network requires a security password.' }}
                    </p>
                </div>
                
                <div class="dialog-body">
                    <!-- Username field (only visible for Enterprise / 802.1X networks) -->
                    <div v-if="isEnterprise" class="form-group" style="margin-bottom: 16px;">
                        <label for="wifi-username">Username / Identity</label>
                        <input 
                            id="wifi-username"
                            type="text" 
                            v-model="username" 
                            placeholder="Enter username" 
                            class="flat-input"
                            @keyup.enter="handleConnect"
                            autofocus
                        />
                    </div>

                    <!-- Password field -->
                    <div class="form-group">
                        <label for="wifi-password">Password</label>
                        <div class="password-input-wrapper">
                            <input 
                                id="wifi-password"
                                :type="showPassword ? 'text' : 'password'" 
                                v-model="password" 
                                placeholder="Enter network password" 
                                class="flat-input"
                                @keyup.enter="handleConnect"
                                :autofocus="!isEnterprise"
                            />
                            <button class="visibility-toggle" @click="toggleShowPassword" type="button">
                                <i class="pi" :class="showPassword ? 'pi-eye-slash' : 'pi-eye'"></i>
                            </button>
                        </div>
                        <span v-if="errorMsg" class="error-hint">
                            <i class="pi pi-exclamation-circle"></i> {{ errorMsg }}
                        </span>
                    </div>
                </div>

                <div class="dialog-actions">
                    <button class="btn-cancel" @click="closeDialog" :disabled="saving">Cancel</button>
                    <button 
                        class="btn-confirm" 
                        @click="handleConnect" 
                        :disabled="saving || password.trim().length === 0 || (isEnterprise && username.trim().length === 0)"
                    >
                        <i v-if="saving" class="pi pi-spin pi-spinner"></i>
                        {{ saving ? 'Connecting...' : 'Connect' }}
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
    width: 380px;
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

.form-group {
    margin-bottom: 0;
}

.form-group label {
    display: block;
    margin-bottom: 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.password-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
}

.flat-input {
    width: 100%;
    padding: 10px 42px 10px 12px;
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

.visibility-toggle {
    position: absolute;
    right: 8px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
}

.visibility-toggle:hover {
    color: var(--text-primary);
    background: rgba(0,0,0,0.05);
}

:global(.dark) .visibility-toggle:hover {
    background: rgba(255,255,255,0.05);
}

.error-hint {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 8px;
    font-size: 11px;
    color: #ff6b6b;
}

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

:global(.dark) .btn-cancel:hover {
    background: rgba(255,255,255,0.05);
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

.btn-confirm:hover:not(:disabled) {
    opacity: 0.9;
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
</style>
