/**
 * Appearance ViewModel (Composable)
 * Contains business logic for appearance settings (theme, wallpaper, waybar)
 */

import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { AppSettings } from '../models/appearance.model';
import { useToast } from '../composables/useToast';

export function useAppearanceViewModel() {
    // --- State ---
    const loading = ref(false);
    const isDark = ref(false);
    const currentWallpaperSrc = ref('');
    const waybarPosition = ref('bottom');
    const changingPosition = ref(false);

    // Notifications
    const { showToast } = useToast();

    // --- Appearance State ---
    const cursorThemes = ref<Array<{ name: string; path: string }>>([]);
    const gtkThemes = ref<Array<{ name: string; path: string }>>([]);

    const selectedCursorTheme = ref('');
    const selectedCursorSize = ref(24);
    const selectedGtkTheme = ref('');

    // Common cursor sizes
    const cursorSizes = [16, 20, 22, 24, 28, 32, 36, 40, 48, 64];

    // --- Actions ---

    // Load initial data
    const loadAppearanceData = async () => {
        try {
            // Load lists
            cursorThemes.value = await invoke('get_cursor_themes');
            gtkThemes.value = await invoke('get_gtk_themes_list');

            // Load current config
            const config = await invoke<{
                cursor_theme: string;
                cursor_size: number;
                gtk_theme: string;
                color_scheme: string;
            }>('get_current_appearance_config');

            selectedCursorTheme.value = config.cursor_theme;
            selectedCursorSize.value = config.cursor_size || 24;
            selectedGtkTheme.value = config.gtk_theme; // Note: user requested filtering from .themes, but current might be system default not in that folder.

            // Sync dark mode state if needed (optional, as we already have settings logic)
            // But we should respect what the backend says the *actual* system state is
            isDark.value = config.color_scheme === 'prefer-dark';
            applyThemeClass(isDark.value);

        } catch (e) {
            console.error('Failed to load appearance data:', e);
            showToast('Failed to load appearance settings', 'error');
        }
    };

    // Apply settings
    const applyAppearanceSettings = async () => {
        try {
            await invoke('apply_appearance_conf', {
                cursorTheme: selectedCursorTheme.value,
                cursorSize: selectedCursorSize.value,
                gtkTheme: selectedGtkTheme.value,
                darkMode: isDark.value
            });
            showToast('Appearance settings applied successfully', 'success');
        } catch (e: any) {
            console.error('Failed to apply settings:', e);
            showToast('Failed to apply settings: ' + e, 'error');
        }
    };

    // Theme Logic
    const toggleTheme = async () => {
        isDark.value = !isDark.value;
        const newTheme = isDark.value ? 'dark' : 'light';
        applyThemeClass(isDark.value);

        // We apply immediately to system now via our new function
        await applyAppearanceSettings();

        // Keep legacy settings save for app-internal persistence if needed
        try {
            const currentSettings = await invoke<AppSettings>('get_app_settings');
            await invoke('save_app_settings', {
                settings: {
                    ...currentSettings,
                    theme: newTheme
                }
            });
        } catch (e) {
            console.error('Save settings error:', e);
        }
    };

    const applyThemeClass = (dark: boolean) => {
        if (dark) {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');
        }
    };

    // Wallpaper Logic
    const updatePreview = async () => {
        try {
            const b64 = await invoke<string>('get_wallpaper_base64');
            currentWallpaperSrc.value = `data:image/png;base64,${b64}`;
        } catch (e) {
            handleImageError();
        }
    };

    const handleImageError = () => {
        currentWallpaperSrc.value = 'https://via.placeholder.com/400x300?text=No+Wallpaper';
    };

    const pickWallpaper = async () => {
        try {
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'Images',
                    extensions: ['png', 'jpg', 'jpeg', 'webp']
                }]
            });

            if (selected) {
                loading.value = true;
                await invoke('set_wallpaper', { filePath: selected });
                updatePreview();
                showToast('Wallpaper updated successfully', 'success');
            }
        } catch (e: any) {
            console.error('Failed to set wallpaper:', e);
            showToast('Failed to set wallpaper: ' + e, 'error');
        } finally {
            loading.value = false;
        }
    };

    // Waybar Position Logic
    const setWaybarPosition = async (position: string) => {
        if (changingPosition.value || position === waybarPosition.value) return;

        changingPosition.value = true;
        try {
            // Set waybar position (copy config files and reload waybar)
            await invoke('set_waybar_position', { position });
            waybarPosition.value = position;

            // Save to settings
            const currentSettings = await invoke<AppSettings>('get_app_settings');
            await invoke('save_app_settings', {
                settings: {
                    ...currentSettings,
                    waybar_position: position
                }
            });

            showToast(`Taskbar position set to ${position}`, 'success');
        } catch (e: any) {
            console.error('Failed to set waybar position:', e);
            showToast('Failed to set taskbar position: ' + e, 'error');
        } finally {
            changingPosition.value = false;
        }
    };

    // --- Lifecycle ---
    onMounted(async () => {
        await loadAppearanceData();

        try {
            const settings = await invoke<AppSettings>('get_app_settings');
            if (settings.waybar_position) {
                waybarPosition.value = settings.waybar_position;
            }
        } catch (e) {
            console.error('Failed to load app settings:', e);
        }

        // Initial Preview Load
        updatePreview();
    });

    return {
        // State
        loading,
        isDark,
        currentWallpaperSrc,
        waybarPosition,
        changingPosition,

        // New State
        cursorThemes,
        gtkThemes,
        selectedCursorTheme,
        selectedCursorSize,
        selectedGtkTheme,
        cursorSizes,

        // Actions
        toggleTheme,
        pickWallpaper,
        handleImageError,
        setWaybarPosition,
        applyAppearanceSettings
    };
}
