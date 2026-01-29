<script setup lang="ts">
import type { Monitor } from '@/models/display.model';
import { TRANSFORM_OPTIONS } from '@/models/display.model';

interface Props {
    monitor: Monitor;
    index: number;
    isDragging: boolean;
    isDragOver: boolean;
    availableMonitors: Monitor[];
}

interface Emits {
    (e: 'dragstart', event: DragEvent, index: number): void;
    (e: 'dragenter', index: number): void;
    (e: 'dragover', event: DragEvent): void;
    (e: 'drop', event: DragEvent, index: number): void;
    (e: 'dragend'): void;
    (e: 'remove', index: number): void;
    (e: 'mirror-focus', monitor: Monitor): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const formatModelName = (monitor: Monitor) => {
    return monitor.model || monitor.name;
};

const validateInteger = (evt: KeyboardEvent) => {
    // Allow navigation keys and control keys
    if (['Backspace', 'Delete', 'ArrowLeft', 'ArrowRight', 'Tab', 'Enter', 'Escape'].includes(evt.key) ||
        (evt.ctrlKey || evt.metaKey)) {
        return;
    }
    // Prevent non-numeric keys
    if (!/^[0-9]$/.test(evt.key)) {
        evt.preventDefault();
    }
};

const validateFloat = (evt: KeyboardEvent) => {
    // Allow navigation, control, and decimal point
    if (['Backspace', 'Delete', 'ArrowLeft', 'ArrowRight', 'Tab', 'Enter', 'Escape', '.'].includes(evt.key) ||
        (evt.ctrlKey || evt.metaKey)) {
        return;
    }
    // Prevent non-numeric keys
    if (!/^[0-9]$/.test(evt.key)) {
        evt.preventDefault();
    }
};
</script>

<template>
    <div 
        class="p-card monitor-card"
        :draggable="true"
        @dragstart="emit('dragstart', $event, index)"
        @dragenter="emit('dragenter', index)"
        @dragover.prevent="emit('dragover', $event)"
        @drop="emit('drop', $event, index)"
        @dragend="emit('dragend')"
        :class="{ 
            'is-dragging': isDragging,
            'is-drag-over': isDragOver
        }"
    >
        <!-- Card Header -->
        <div class="p-card-header">
            <div class="monitor-icon">
                <i class="pi pi-desktop"></i>
            </div>
            <div class="monitor-title">
                <h2>{{ formatModelName(monitor) }}</h2>
                <span class="monitor-subtitle">{{ monitor.name }} (ID: {{ monitor.id }})</span>
            </div>
            <div class="header-controls">
                <button class="p-button p-button-danger p-button-sm" @click="emit('remove', index)">
                    <i class="pi pi-trash"></i>
                    Remove
                </button>
                <label class="switch">
                    <input type="checkbox" :checked="monitor.enabled" @change="monitor.enabled = !monitor.enabled">
                    <span class="slider round"></span>
                </label>
            </div>
        </div>
        
        <!-- Card Content -->
        <div class="p-card-content">
            <div class="form-grid">
                <!-- Resolution Group -->
                <div class="form-group full-width">
                    <label>Resolution & Rate</label>
                    <div class="p-inputgroup">
                        <input type="number" v-model="monitor.width" class="p-inputtext" placeholder="Width" @keydown="validateInteger" />
                        <span class="p-inputgroup-addon">×</span>
                        <input type="number" v-model="monitor.height" class="p-inputtext" placeholder="Height" @keydown="validateInteger" />
                        <span class="p-inputgroup-addon">@</span>
                        <input type="number" v-model="monitor.refreshRate" class="p-inputtext" step="0.001" placeholder="Hz" @keydown="validateFloat" />
                        <span class="p-inputgroup-addon">Hz</span>
                    </div>
                </div>

                <!-- Position Group -->
                <div class="form-group">
                    <label>Position (X, Y)</label>
                    <div class="p-inputgroup">
                        <input type="number" v-model="monitor.x" class="p-inputtext" placeholder="X" @keydown="validateInteger" />
                        <span class="p-inputgroup-addon">×</span>
                        <input type="number" v-model="monitor.y" class="p-inputtext" placeholder="Y" @keydown="validateInteger" />
                    </div>
                </div>

                <!-- Scale Group -->
                <div class="form-group">
                    <label>Scale</label>
                    <div class="p-inputgroup">
                        <input type="number" v-model="monitor.scale" class="p-inputtext" step="0.1" placeholder="Scale" @keydown="validateFloat" />
                        <span class="p-inputgroup-addon">x</span>
                    </div>
                </div>

                <!-- Display Mode Selector -->
                <div class="form-group full-width" v-if="monitor.enabled">
                    <label>Display Mode</label>
                    <div class="display-mode-container">
                        <div class="mode-tabs">
                            <button 
                                type="button"
                                class="mode-tab"
                                :class="{ active: monitor.mirror === null }"
                                @click="monitor.mirror = null"
                            >
                                <i class="pi pi-desktop"></i>
                                <span>Extend</span>
                            </button>
                            <button 
                                type="button"
                                class="mode-tab"
                                :class="{ active: monitor.mirror !== null }"
                                @click="monitor.mirror = monitor.mirror !== null ? monitor.mirror : ''"
                            >
                                <i class="pi pi-clone"></i>
                                <span>Mirror</span>
                            </button>
                        </div>
                        <select 
                            v-if="monitor.mirror !== null || availableMonitors.length > 0"
                            v-model="monitor.mirror" 
                            class="mirror-dropdown"
                            @focus="emit('mirror-focus', monitor)"
                        >
                            <option value="" disabled>Select monitor to mirror</option>
                            <option 
                                v-for="m in availableMonitors" 
                                :key="m.name"
                                :value="m.name"
                            >
                                {{ m.model || m.name }}
                            </option>
                        </select>
                    </div>
                </div>

                <!-- Orientation Group -->
                <div class="form-group full-width" v-if="monitor.enabled">
                    <label>Orientation (Transform)</label>
                    <div class="orientation-selector">
                        <button 
                            v-for="option in TRANSFORM_OPTIONS" 
                            :key="option.value"
                            type="button"
                            class="orientation-option"
                            :class="{ active: monitor.transform === option.value }"
                            @click="monitor.transform = option.value"
                            :title="option.title"
                        >
                            <i :class="['pi', option.icon]" :style="option.iconStyle"></i>
                            <span>{{ option.label }}</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Component-specific styles will be inherited from parent */
</style>
