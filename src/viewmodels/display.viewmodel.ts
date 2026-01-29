/**
 * Display ViewModel (Composable)
 * Contains all business logic for display management
 */

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Monitor } from '../models/display.model';
import { useToast } from '../composables/useToast';

export function useDisplayViewModel() {
    // State
    const monitors = ref<Monitor[]>([]);
    const loading = ref(true);
    const draggingCardIndex = ref<number | null>(null);
    const dragOverIndex = ref<number | null>(null);

    // Notifications
    const { showToast } = useToast();

    /**
     * Fetch monitors from backend
     */
    async function fetchMonitors(): Promise<void> {
        loading.value = true;
        try {
            const fetchedMonitors = await invoke<Monitor[]>('get_displays');
            // Initialize enabled and mirror fields if not present
            monitors.value = fetchedMonitors.map(m => ({
                ...m,
                enabled: m.enabled !== undefined ? m.enabled : true,
                mirror: m.mirror !== undefined ? m.mirror : null
            }));
        } catch (error) {
            showToast('Failed to fetch monitors: ' + error, 'error');
        } finally {
            loading.value = false;
        }
    }

    /**
     * Format monitor model name
     */
    function formatModelName(monitor: Monitor): string {
        return monitor.model || monitor.name;
    }

    /**
     * Remove a monitor
     */
    function removeMonitor(index: number): void {
        monitors.value.splice(index, 1);
        recalculateMonitorPositions();
    }

    /**
     * Drag and drop handlers
     */
    function onCardDragStart(event: DragEvent, index: number): void {
        draggingCardIndex.value = index;
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = 'move';
        }
    }

    function onCardDragEnter(index: number): void {
        if (draggingCardIndex.value !== null && draggingCardIndex.value !== index) {
            dragOverIndex.value = index;
        }
    }

    function onCardDrop(event: DragEvent, dropIndex: number): void {
        event.preventDefault();
        const dragIndex = draggingCardIndex.value;

        if (dragIndex !== null && dragIndex !== dropIndex) {
            const draggedItem = monitors.value[dragIndex];
            monitors.value.splice(dragIndex, 1);
            monitors.value.splice(dropIndex, 0, draggedItem);
            recalculateMonitorPositions();
        }
    }

    function onCardDragEnd(): void {
        draggingCardIndex.value = null;
        dragOverIndex.value = null;
    }

    /**
     * Recalculate monitor positions based on order
     */
    function recalculateMonitorPositions(): void {
        let currentX = 0;

        monitors.value.forEach(monitor => {
            monitor.x = currentX;
            monitor.y = 0;

            // Logical width approximation
            const logicalWidth = Math.ceil(monitor.width / monitor.scale);
            currentX += logicalWidth;
        });
    }

    /**
     * Handle mirror dropdown focus
     */
    function onMirrorDropdownFocus(monitor: Monitor): void {
        if (monitor.mirror === null) {
            monitor.mirror = '';
        }
    }

    /**
     * Get available monitors for mirroring
     */
    function getAvailableMonitorsForMirror(currentMonitor: Monitor): Monitor[] {
        return monitors.value.filter(m =>
            m.name !== currentMonitor.name && m.enabled
        );
    }

    /**
     * Save display settings
     */
    async function saveSettings(): Promise<void> {
        try {
            loading.value = true;

            // Backend now handles everything - generates and saves Hyprland config directly
            await invoke('save_displays', { monitors: monitors.value });

            showToast('Settings saved to ~/.config/hypr/configs/monitors.conf', 'success');
        } catch (error) {
            showToast('Failed to save settings: ' + error, 'error');
        } finally {
            loading.value = false;
        }
    }

    // Return all state and methods
    return {
        // State
        monitors,
        loading,
        draggingCardIndex,
        dragOverIndex,

        // Methods
        fetchMonitors,
        formatModelName,
        removeMonitor,
        onCardDragStart,
        onCardDragEnter,
        onCardDrop,
        onCardDragEnd,
        recalculateMonitorPositions,
        onMirrorDropdownFocus,
        getAvailableMonitorsForMirror,
        saveSettings
    };
}
