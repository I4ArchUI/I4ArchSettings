/**
 * Keybinds ViewModel (Composable)
 * Logic for managing Hyprland Keybinds
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

export interface Keybind {
    id: string;
    bind_type: string;
    modifiers: string;
    key: string;
    dispatcher: string;
    args: string;
}

export function useKeybindsViewModel() {
    const loading = ref(false);
    const keybinds = ref<Keybind[]>([]);
    const { showToast } = useToast();

    // Bind Types
    const bindTypes = ref([
        { label: 'Normal', value: 'bind' },
        { label: 'Repeat', value: 'binde' },
        { label: 'Mouse', value: 'bindm' },
        { label: 'Locked', value: 'bindl' },
        { label: 'Locked+Repeat', value: 'bindel' },
        { label: 'Locked+Mouse', value: 'bindle' },
        { label: 'Release', value: 'bindr' },
        { label: 'Transparent', value: 'bindt' },
        { label: 'Ignore Mods', value: 'bindi' }
    ]);

    // Standard modifiers
    const modifierOptions = [
        { label: 'Super', value: 'SUPER' },
        { label: 'Super + Shift', value: 'SUPER SHIFT' },
        { label: 'Super + Ctrl', value: 'SUPER CTRL' },
        { label: 'Super + Alt', value: 'SUPER ALT' },
        { label: 'Ctrl + Alt', value: 'CTRL ALT' },
        { label: 'Alt', value: 'ALT' },
        { label: 'None', value: '' }
    ];

    const loadKeybinds = async () => {
        loading.value = true;
        try {
            const result = await invoke<Keybind[]>('get_keybinds');
            keybinds.value = result.map(kb => {
                const type = kb.bind_type || 'bind';

                if (!bindTypes.value.find(t => t.value === type)) {
                    bindTypes.value.push({
                        label: `${type}`,
                        value: type
                    });
                }

                return {
                    ...kb,
                    bind_type: type,
                    modifiers: kb.modifiers ? kb.modifiers.toUpperCase() : ''
                };
            });
        } catch (e: any) {
            console.error('Failed to load keybinds:', e);
            showToast('Failed to load keybinds', 'error');
        } finally {
            loading.value = false;
        }
    };

    const saveKeybinds = async () => {
        loading.value = true;
        try {
            await invoke('save_keybinds', {
                args: {
                    keybinds: keybinds.value
                }
            });
            await loadKeybinds();
            showToast('Keybinds saved successfully', 'success');
        } catch (e: any) {
            showToast('Failed to save keybinds', 'error');
        } finally {
            loading.value = false;
        }
    };

    const addKeybind = () => {
        keybinds.value.push({
            id: `new-${Date.now()}`,
            bind_type: 'bind',
            modifiers: 'SUPER',
            key: '',
            dispatcher: 'exec',
            args: ''
        });
    };

    const removeKeybind = (index: number) => {
        keybinds.value.splice(index, 1);
    };

    onMounted(() => {
        loadKeybinds();
    });

    return {
        loading,
        keybinds,
        modifierOptions,
        bindTypes,
        loadKeybinds,
        saveKeybinds,
        addKeybind,
        removeKeybind
    };
}
