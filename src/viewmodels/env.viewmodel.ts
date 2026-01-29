/**
 * Environment Variables ViewModel (Composable)
 * Logic for managing Hyprland Environment Variables
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

export interface EnvVar {
    id: string;
    key: string;
    value: string;
}

export function useEnvViewModel() {
    const loading = ref(false);
    const envVars = ref<EnvVar[]>([]);
    const { showToast } = useToast();

    const loadEnvVars = async () => {
        loading.value = true;
        try {
            const result = await invoke<EnvVar[]>('get_env_vars');

            // Ensure unique IDs if coming from backend without them (though our rust backend provides them)
            // If empty, provide one empty row
            if (result.length === 0) {
                envVars.value = [{
                    id: `env-${Date.now()}`,
                    key: '',
                    value: ''
                }];
            } else {
                envVars.value = result;
            }
        } catch (e: any) {
            showToast('Failed to load environment variables', 'error');
            // Fallback to empty row
            envVars.value = [{
                id: `env-${Date.now()}`,
                key: '',
                value: ''
            }];
        } finally {
            loading.value = false;
        }
    };

    const saveEnvVars = async () => {
        loading.value = true;
        try {
            // Filter out empty keys
            const toSave = envVars.value.filter(v => v.key.trim() !== '');

            await invoke('save_env_vars', { vars: toSave });
            await loadEnvVars();
            showToast('Environment variables saved successfully', 'success');
        } catch (e: any) {
            showToast('Failed to save environment variables', 'error');
        } finally {
            loading.value = false;
        }
    };

    const addEnvVar = () => {
        envVars.value.unshift({
            id: `new-${Date.now()}`,
            key: '',
            value: ''
        });
    };

    const removeEnvVar = (index: number) => {
        envVars.value.splice(index, 1);
        if (envVars.value.length === 0) {
            addEnvVar();
        }
    };

    onMounted(() => {
        loadEnvVars();
    });

    return {
        loading,
        envVars,
        loadEnvVars,
        saveEnvVars,
        addEnvVar,
        removeEnvVar
    };
}
