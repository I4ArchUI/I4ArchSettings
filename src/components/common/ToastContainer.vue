<script setup lang="ts">
import { useToast } from '@/composables/useToast';

const { toasts, removeToast } = useToast();
</script>

<template>
    <div class="toast-container">
        <TransitionGroup name="toast">
            <div 
                v-for="toast in toasts" 
                :key="toast.id" 
                class="toast-message"
                :class="toast.type"
                @click="removeToast(toast.id)"
            >
                <div class="toast-icon">
                    <i v-if="toast.type === 'success'" class="pi pi-check-circle"></i>
                    <i v-else-if="toast.type === 'error'" class="pi pi-times-circle"></i>
                    <i v-else-if="toast.type === 'warning'" class="pi pi-exclamation-triangle"></i>
                    <i v-else class="pi pi-info-circle"></i>
                </div>
                <div class="toast-content">
                    {{ toast.message }}
                </div>
                <button class="toast-close">
                    <i class="pi pi-times"></i>
                </button>
            </div>
        </TransitionGroup>
    </div>
</template>

<style scoped>
.toast-container {
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 12px;
    pointer-events: none;
}

.toast-message {
    pointer-events: auto;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 20px;
    border-radius: 12px;
    background: #1e1e1e; /* Dark theme default */
    color: white;
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
    min-width: 300px;
    max-width: 450px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    cursor: pointer;
    backdrop-filter: blur(10px);
}

/* Theme Colors */
.toast-message.success {
    background: rgba(22, 163, 74, 0.9);
    border-color: rgba(22, 163, 74, 0.5);
}

.toast-message.error {
    background: rgba(220, 38, 38, 0.9);
    border-color: rgba(220, 38, 38, 0.5);
}

.toast-message.warning {
    background: rgba(202, 138, 4, 0.9);
    border-color: rgba(202, 138, 4, 0.5);
}

.toast-message.info {
    background: rgba(37, 99, 235, 0.9);
    border-color: rgba(37, 99, 235, 0.5);
}

.toast-icon {
    font-size: 1.25rem;
    display: flex;
    align-items: center;
}

.toast-message.success .toast-icon { color: #4ade80; }
.toast-message.error .toast-icon { color: #f87171; }
.toast-message.warning .toast-icon { color: #fcd34d; }
.toast-message.info .toast-icon { color: #60a5fa; }

.toast-content {
    flex: 1;
    font-size: 0.95rem;
    font-weight: 500;
    line-height: 1.4;
}

.toast-close {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.5);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    transition: all 0.2s;
}

.toast-close:hover {
    background: rgba(255, 255, 255, 0.1);
    color: white;
}

/* Animations */
.toast-enter-active,
.toast-leave-active {
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.toast-enter-from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
}

.toast-leave-to {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
}
</style>
