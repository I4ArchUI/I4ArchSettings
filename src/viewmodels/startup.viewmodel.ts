/**
 * Startup ViewModel (Composable)
 * Logic for managing startup applications
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

export interface StartupCommand {
    id: string;
    command: string;
}

export function useStartupViewModel() {
    const loading = ref(false);
    const startupCommands = ref<StartupCommand[]>([]);
    const { showToast } = useToast();

    // Load commands from backend
    const loadCommands = async () => {
        loading.value = true;
        try {
            const commands = await invoke<string[]>('get_startup_commands');
            // Convert strings to objects with IDs
            if (commands.length === 0) {
                startupCommands.value = [{ id: `cmd-${Date.now()}`, command: '' }];
            } else {
                startupCommands.value = commands.map((cmd, index) => ({
                    id: `cmd-${Date.now()}-${index}`,
                    command: cmd
                }));
            }
        } catch (e: any) {
            showToast('Failed to load startup commands', 'error');
        } finally {
            loading.value = false;
        }
    };

    // Save commands to backend
    const saveCommands = async () => {
        loading.value = true;
        try {
            // Filter out empty lines before saving and extract command string
            const toSave = startupCommands.value
                .map(item => item.command)
                .filter(cmd => cmd.trim() !== '');

            await invoke('save_startup_commands', { commands: toSave });

            // Reload to reflect clean state (optional, but good practice)
            await loadCommands();
            showToast('Startup commands saved successfully', 'success');
        } catch (e: any) {
            showToast('Failed to save startup commands', 'error');
        } finally {
            loading.value = false;
        }
    };

    // Add a new empty command row
    const addCommand = () => {
        startupCommands.value.unshift({
            id: `new-${Date.now()}`,
            command: ''
        });
    };

    // Remove a command row
    const removeCommand = (index: number) => {
        startupCommands.value.splice(index, 1);
        // If all removed, keep one empty row
        if (startupCommands.value.length === 0) {
            startupCommands.value.push({
                id: `new-${Date.now()}`,
                command: ''
            });
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
