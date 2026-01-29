<script setup lang="ts">
/**
 * DisplaysView - View Layer (Component-Based)
 * Uses child components for better organization
 */

import { onMounted } from 'vue';
import { useDisplayViewModel } from '../viewmodels/display.viewmodel';
import MonitorCard from '@/components/display/MonitorCard.vue';
import LoadingState from '@/components/LoadingState.vue';

// Initialize ViewModel using composable
const {
    // State
    monitors,
    loading,
    draggingCardIndex,
    dragOverIndex,
    
    // Methods
    fetchMonitors,
    removeMonitor,
    onCardDragStart,
    onCardDragEnter,
    onCardDrop,
    onCardDragEnd,
    onMirrorDropdownFocus,
    getAvailableMonitorsForMirror,
    saveSettings
} = useDisplayViewModel();

// Lifecycle
onMounted(async () => {
    await fetchMonitors();
});
</script>

<template>
    <div class="displays-view">
        <!-- Header Component -->
        <div class="header">
            <h1 class="page-title">Displays</h1>
            <div class="actions">
                <button class="p-button primary-btn" @click="saveSettings">
                    <i class="pi pi-save"></i> Save
                </button>
                <button class="p-button-icon-only refresh-btn" @click="fetchMonitors" title="Refresh">
                    <i class="pi pi-refresh" :class="{ 'pi-spin': loading }"></i>
                </button>
            </div>
        </div>

        <!-- Loading State Component -->
        <LoadingState v-if="loading && monitors.length === 0" />

        <!-- Monitors Grid with MonitorCard Components -->
        <div v-else class="monitors-grid">
            <MonitorCard
                v-for="(monitor, index) in monitors"
                :key="monitor.id"
                :monitor="monitor"
                :index="index"
                :is-dragging="draggingCardIndex === index"
                :is-drag-over="dragOverIndex === index"
                :available-monitors="getAvailableMonitorsForMirror(monitor)"
                @dragstart="onCardDragStart"
                @dragenter="onCardDragEnter"
                @dragover="(e) => e.preventDefault()"
                @drop="onCardDrop"
                @dragend="onCardDragEnd"
                @remove="removeMonitor"
                @mirror-focus="onMirrorDropdownFocus"
            />
        </div>
    </div>
</template>

<style scoped>
.header {
    padding: 20px 0 30px 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.page-title {
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--text-primary);
}

.actions {
    display: flex;
    gap: 12px;
}

.p-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    font-size: 0.9rem;
    border: none;
    transition: all 0.2s;
}

.primary-btn {
    background: var(--accent-color);
    color: white;
}

.primary-btn:hover {
    filter: brightness(1.1);
}

.refresh-btn {
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: color 0.2s ease;
}

.refresh-btn:hover {
    color: var(--text-primary);
}

.displays-view {
    padding: 0 40px 40px 40px;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
    box-sizing: border-box;
}

.monitors-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 24px;
}

@media (max-width: 768px) {
    .monitors-grid {
        grid-template-columns: 1fr;
    }
}

/* Global styles for monitor cards */
:deep(.p-card) {
    background: var(--card-bg, #ffffff);
    border-radius: 12px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    border: 1px solid var(--card-border, #e5e7eb);
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 24px;
    transition: transform 0.2s, box-shadow 0.2s;
    cursor: grab;
}

:deep(.p-card:hover) {
    transform: translateY(-2px);
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

:deep(.p-card:active) {
    cursor: grabbing;
}

:deep(.p-card.is-dragging) {
    opacity: 0.4;
    border: 2px dashed var(--accent-color);
}

:deep(.p-card.is-drag-over) {
    transform: scale(1.02);
    box-shadow: 0 0 0 2px var(--accent-color);
    z-index: 10;
}

:deep(.p-card-header) {
    display: flex;
    align-items: flex-start;
    gap: 16px;
}

:deep(.monitor-icon) {
    width: 48px;
    height: 48px;
    background: var(--bg-secondary);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent-color);
}

:deep(.monitor-icon i) {
    font-size: 1.5rem;
}

:deep(.monitor-title) {
    flex: 1;
}

:deep(.monitor-title h2) {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 0 0 4px 0;
    color: var(--text-primary);
}

:deep(.monitor-subtitle) {
    font-size: 0.85rem;
    color: var(--text-secondary);
}

:deep(.header-controls) {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-left: auto;
}

:deep(.status-toggle-compact) {
    display: flex;
    gap: 4px;
    padding: 3px;
    background: var(--bg-secondary);
    border-radius: 6px;
    border: 1px solid var(--card-border);
}

:deep(.status-btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--text-secondary);
}

:deep(.status-btn i) {
    font-size: 1.1rem;
}

:deep(.status-btn:hover:not(.active)) {
    background: var(--card-bg);
    color: var(--text-primary);
}

:deep(.status-btn.active) {
    background: var(--accent-color);
    color: white;
    box-shadow: 0 1px 4px rgba(0, 122, 255, 0.3);
}

:deep(.form-grid) {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
}

:deep(.full-width) {
    grid-column: 1 / -1;
}

:deep(.form-group) {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

:deep(.form-group label) {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary);
}

:deep(.p-inputgroup) {
    display: flex;
    align-items: stretch;
    width: 100%;
}

:deep(.p-inputtext) {
    flex: 1;
    font-family: inherit;
    font-size: 0.9rem;
    color: var(--text-primary);
    background: var(--bg-secondary);
    padding: 0.75rem 0.75rem;
    border: 1px solid var(--card-border);
    appearance: none;
    transition: all 0.2s;
    min-width: 0;
}

:deep(.p-inputtext:focus) {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 1px var(--accent-color);
    z-index: 1;
    background: var(--card-bg);
}

:deep(.p-inputgroup .p-inputtext:first-child) {
    border-radius: 6px 0 0 6px;
}

:deep(.p-inputgroup .p-inputtext:last-child) {
    border-radius: 0 6px 6px 0;
}

:deep(.p-inputgroup .p-inputtext:not(:first-child):not(:last-child)) {
    border-radius: 0;
    margin-left: -1px;
}

:deep(.p-inputgroup-addon) {
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--card-border);
    padding: 0 0.75rem;
    min-width: 2.5rem;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.85rem;
    font-weight: 500;
}

:deep(.p-inputgroup-addon:first-child) {
    border-radius: 6px 0 0 6px;
    border-right: 0;
}

:deep(.p-inputgroup-addon:last-child) {
    border-radius: 0 6px 6px 0;
    border-left: 0;
}

:deep(.p-inputgroup > * + *) {
    margin-left: -1px;
}

:deep(.p-inputgroup-addon:not(:first-child):not(:last-child)) {
    border-radius: 0;
    border-left: 0;
    border-right: 0;
    border: 1px solid var(--card-border);
}

:deep(.p-inputtext:hover) {
    border-color: var(--text-secondary);
    z-index: 1;
}

:deep(select.p-inputtext) {
    appearance: auto;
}

:deep(.orientation-selector) {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
    padding: 4px;
    background: var(--bg-secondary);
    border-radius: 8px;
    border: 1px solid var(--card-border);
}

:deep(.orientation-option) {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 10px 6px;
    background: transparent;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--text-secondary);
    font-size: 0.7rem;
    font-weight: 500;
}

:deep(.orientation-option i) {
    font-size: 1.1rem;
    transition: transform 0.2s ease;
}

:deep(.orientation-option:hover) {
    background: var(--card-bg);
    color: var(--text-primary);
}

:deep(.orientation-option.active) {
    background: var(--accent-color);
    color: white;
    box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
}

:deep(.orientation-option.active i) {
    color: white;
}

:deep(.orientation-option span) {
    font-size: 0.65rem;
    font-weight: 600;
    letter-spacing: 0.2px;
    white-space: nowrap;
}

:deep(.display-mode-container) {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

:deep(.mode-tabs) {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    padding: 4px;
    background: var(--bg-secondary);
    border-radius: 8px;
    border: 1px solid var(--card-border);
}

:deep(.mode-tab) {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px 16px;
    background: transparent;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 500;
}

:deep(.mode-tab i) {
    font-size: 1rem;
}

:deep(.mode-tab:hover:not(.active)) {
    background: var(--card-bg);
    color: var(--text-primary);
}

:deep(.mode-tab.active) {
    background: var(--accent-color);
    color: white;
    box-shadow: 0 2px 6px rgba(0, 122, 255, 0.25);
}

:deep(.mirror-dropdown) {
    width: 100%;
    padding: 12px 16px;
    border: 1px solid var(--card-border);
    border-radius: 8px;
    background: var(--card-bg);
    color: var(--text-primary);
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    outline: none;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23666' d='M6 9L1 4h10z'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 36px;
}

:deep(.mirror-dropdown:hover) {
    border-color: var(--accent-color);
    background-color: var(--bg-secondary);
}

:deep(.mirror-dropdown:focus) {
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
}

:deep(.mirror-dropdown option) {
    padding: 10px;
    background: var(--card-bg);
    color: var(--text-primary);
}

:deep(.p-card-footer) {
    display: flex;
    justify-content: flex-end;
    padding-top: 12px;
    border-top: 1px solid var(--card-border);
}

:deep(.p-button-danger) {
    background: #ef4444;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    transition: all 0.2s;
}

:deep(.p-button-danger:hover) {
    background: #dc2626;
}

:deep(.p-button-sm) {
    padding: 6px 12px;
    font-size: 0.8rem;
}
</style>
