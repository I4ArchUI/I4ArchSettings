<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';

defineProps<{
    title: string;
    modelValue: boolean; // open/close state
}>();

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void;
    (e: 'close'): void;
}>();

const close = () => {
    emit('update:modelValue', false);
    emit('close');
};

// Close on escape
const onKeydown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') close();
};

onMounted(() => document.addEventListener('keydown', onKeydown));
onUnmounted(() => document.removeEventListener('keydown', onKeydown));
</script>

<template>
    <div v-if="modelValue" class="modal-backdrop" @click.self="close">
        <div class="modal-container glass-panel">
            <div class="modal-header">
                <h3>{{ title }}</h3>
                <button class="close-btn" @click="close">
                    <i class="pi pi-times"></i>
                </button>
            </div>
            
            <div class="modal-content">
                <slot></slot>
            </div>
            
            <div class="modal-footer" v-if="$slots.footer">
                <slot name="footer"></slot>
            </div>
        </div>
    </div>
</template>

<style scoped>
.modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal-container {
    width: 100%;
    max-width: 500px;
    background-color: var(--card-bg); /* Fallback */
    border-radius: 12px;
    border: 1px solid var(--card-border);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    animation: popIn 0.2s ease-out;
}

.modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid var(--separator-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.modal-header h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
}

.close-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
}

.close-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
}

.modal-content {
    padding: 20px;
}

.modal-footer {
    padding: 16px 20px;
    border-top: 1px solid var(--separator-color);
    display: flex;
    justify-content: flex-end;
    gap: 10px;
}

@keyframes popIn {
    from {
        opacity: 0;
        transform: scale(0.95);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}
</style>
