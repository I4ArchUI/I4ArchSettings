
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface UpdatePackage {
    name: string;
    old_version: string;
    new_version: string;
}

export function useUpdatesViewModel() {
    const updates = ref<UpdatePackage[]>([]);
    const loading = ref(false);
    const error = ref<string | null>(null);

    const checkUpdates = async () => {
        loading.value = true;
        error.value = null;
        try {
            updates.value = await invoke<UpdatePackage[]>('check_updates');
        } catch (e: any) {
            error.value = 'Failed to check for updates: ' + e;
        } finally {
            loading.value = false;
        }
    };

    const updateSystem = async () => {
        try {
            await invoke('update_system');
        } catch (e: any) {
            error.value = 'Failed to launch update terminal: ' + e;
        }
    };

    onMounted(() => {
        checkUpdates();
    });

    return {
        updates,
        loading,
        error,
        checkUpdates,
        updateSystem,
    };
}
