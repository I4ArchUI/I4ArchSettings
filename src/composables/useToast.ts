/**
 * Toast Notification Composable
 * Manages global state for application notifications
 */
import { ref } from 'vue';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface ToastMessage {
    id: number;
    message: string;
    type: ToastType;
    duration?: number;
}

// Global state
const toasts = ref<ToastMessage[]>([]);
let nextId = 0;

export function useToast() {
    /**
     * Show a toast notification
     * @param message The text to display
     * @param type Notification type (success, error, info, warning)
     * @param duration Duration in ms (default 3000)
     */
    const showToast = (message: string, type: ToastType = 'info', duration: number = 3000) => {
        const id = nextId++;
        const toast: ToastMessage = { id, message, type, duration };

        toasts.value.push(toast);

        if (duration > 0) {
            setTimeout(() => {
                removeToast(id);
            }, duration);
        }
    };

    /**
     * Remove a toast by ID
     */
    const removeToast = (id: number) => {
        const index = toasts.value.findIndex(t => t.id === id);
        if (index !== -1) {
            toasts.value.splice(index, 1);
        }
    };

    return {
        toasts,
        showToast,
        removeToast
    };
}
