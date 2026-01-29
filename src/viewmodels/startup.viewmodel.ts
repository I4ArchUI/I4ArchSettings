/**
 * Startup ViewModel (Composable)
 * Logic for managing startup applications
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

export function useStartupViewModel() {
    const loading = ref(false);
    const startupCommands = ref<string[]>([]);
    const { showToast } = useToast();

    // Load commands from backend
    const loadCommands = async () => {
        loading.value = true;
        try {
            const commands = await invoke<string[]>('get_startup_commands');
            // If empty, initialize with one empty string to show at least one input
            if (commands.length === 0) {
                startupCommands.value = [''];
            } else {
                startupCommands.value = commands;
            }
        } catch (e: any) {
            console.error('Failed to load startup commands:', e);
            showToast('Failed to load startup commands', 'error');
        } finally {
            loading.value = false;
        }
    };

    // Save commands to backend
    const saveCommands = async () => {
        loading.value = true;
        try {
            // Filter out empty lines before saving
            const toSave = startupCommands.value.filter(cmd => cmd.trim() !== '');
            await invoke('save_startup_commands', { commands: toSave });

            // Reload to reflect clean state (optional, but good practice)
            await loadCommands();
            showToast('Startup commands saved successfully', 'success');
        } catch (e: any) {
            console.error('Failed to save startup commands:', e);
            showToast('Failed to save startup commands', 'error');
        } finally {
            loading.value = false;
        }
    };

    // Add a new empty command row
    const addCommand = () => {
        startupCommands.value.push('');
    };

    // Remove a command row
    const removeCommand = (index: number) => {
        startupCommands.value.splice(index, 1);
        // If all removed, keep one empty row
        if (startupCommands.value.length === 0) {
            startupCommands.value.push('');
        }
    };

    onMounted(() => {
        loadCommands();
    });

    return {
        loading,
        startupCommands,
        loadCommands,
        saveCommands,
        addCommand,
        removeCommand
    };
}
