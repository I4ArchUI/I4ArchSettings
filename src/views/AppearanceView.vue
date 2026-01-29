<script setup lang="ts">
import { useAppearanceViewModel } from '../viewmodels/appearance.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';

const {
    loading,
    isDark,
    currentWallpaperSrc,
    waybarPosition,
    changingPosition,
    toggleTheme,
    pickWallpaper,
    handleImageError,
    setWaybarPosition,
    cursorThemes,
    gtkThemes,
    selectedCursorTheme,
    selectedCursorSize,
    selectedGtkTheme,
    cursorSizes,
    applyAppearanceSettings,
    hyprlandConfig,
    applyHyprlandConfig
} = useAppearanceViewModel();
</script>

<template>
    <PageLayout>
        <template #title>Appearance</template>

        <div class="card-content-wrapper">
            <div class="image-frame">
                <img :src="currentWallpaperSrc" alt="Current Wallpaper" class="wallpaper-img" @error="handleImageError" />
                
                <div class="change-btn-container">
                    <div class="path-container">
                        <span class="path-highlight">~/.config/hypr/themes/background.png</span>
                    </div>
                    <button class="btn-change" @click="pickWallpaper" :disabled="loading">
                        <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                        <span v-else>Browse Pictures</span>
                    </button>
                </div>
            </div>
        </div>

        <SettingsCard title="Taskbar Position" icon="pi pi-window-maximize">
            <div class="position-buttons" style="padding: 20px;">
                <button 
                    class="position-btn" 
                    :class="{ active: waybarPosition === 'top' }"
                    @click="setWaybarPosition('top')"
                    :disabled="changingPosition"
                >
                    <i class="pi pi-arrow-up"></i>
                    <span>Top</span>
                </button>
                <button 
                    class="position-btn" 
                    :class="{ active: waybarPosition === 'left' }"
                    @click="setWaybarPosition('left')"
                    :disabled="changingPosition"
                >
                    <i class="pi pi-arrow-left"></i>
                    <span>Left</span>
                </button>
                <button 
                    class="position-btn" 
                    :class="{ active: waybarPosition === 'right' }"
                    @click="setWaybarPosition('right')"
                    :disabled="changingPosition"
                >
                    <i class="pi pi-arrow-right"></i>
                    <span>Right</span>
                </button>
                <button 
                    class="position-btn" 
                    :class="{ active: waybarPosition === 'bottom' }"
                    @click="setWaybarPosition('bottom')"
                    :disabled="changingPosition"
                >
                    <i class="pi pi-arrow-down"></i>
                    <span>Bottom</span>
                </button>
            </div>
        </SettingsCard>

        <SettingsCard title="Dark Mode" :icon="isDark ? 'pi pi-moon' : 'pi pi-sun'">
            <template #actions>
                <label class="switch">
                    <input type="checkbox" :checked="isDark" @change="toggleTheme">
                    <span class="slider round"></span>
                </label>
            </template>
        </SettingsCard>

        <SettingsCard title="Cursor" icon="pi pi-stop-circle">
            <div class="settings-row" style="padding: 20px;">
                <div class="setting-control">
                    <label class="setting-label">Theme</label>
                    <select class="custom-select" v-model="selectedCursorTheme" @change="applyAppearanceSettings">
                        <option v-for="theme in cursorThemes" :key="theme.name" :value="theme.name">{{ theme.name }}</option>
                    </select>
                </div>
                <div class="setting-control">
                    <label class="setting-label">Size</label>
                    <select class="custom-select" v-model="selectedCursorSize" @change="applyAppearanceSettings">
                        <option v-for="size in cursorSizes" :key="size" :value="size">{{ size }}px</option>
                    </select>
                </div>
            </div>
        </SettingsCard>

        <SettingsCard title="GTK & Shell Theme" icon="pi pi-palette">
            <div class="settings-row" style="padding: 20px;">
                <div class="setting-control" style="width: 100%;">
                    <select class="custom-select" v-model="selectedGtkTheme" @change="applyAppearanceSettings" style="width: 100%;">
                        <option v-for="theme in gtkThemes" :key="theme.name" :value="theme.name">{{ theme.name }}</option>
                    </select>
                </div>
            </div>
        </SettingsCard>

        <SettingsCard title="Window Layout" icon="pi pi-th-large">
            <template #actions>
                <button class="primary-btn small-btn" @click="applyHyprlandConfig">
                    <i class="pi pi-save"></i> Apply
                </button>
            </template>

            <div class="settings-grid" style="padding: 20px;">
                <div class="setting-control">
                    <label class="setting-label">Gaps In ({{ hyprlandConfig.gaps_in }}px)</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.gaps_in" min="0" max="50">
                </div>
                <div class="setting-control">
                    <label class="setting-label">Gaps Out ({{ hyprlandConfig.gaps_out }}px)</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.gaps_out" min="0" max="50">
                </div>
                <div class="setting-control">
                    <label class="setting-label">Border Size ({{ hyprlandConfig.border_size }}px)</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.border_size" min="0" max="10">
                </div>
                <div class="setting-control">
                    <label class="setting-label">Rounding ({{ hyprlandConfig.rounding }}px)</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.rounding" min="0" max="30">
                </div>
                 <div class="setting-control">
                    <label class="setting-label">Active Opacity ({{ hyprlandConfig.active_opacity }})</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.active_opacity" min="0.1" max="1.0" step="0.05">
                </div>
                <div class="setting-control">
                    <label class="setting-label">Inactive Opacity ({{ hyprlandConfig.inactive_opacity }})</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.inactive_opacity" min="0.1" max="1.0" step="0.05">
                </div>
            </div>
        </SettingsCard>

        <SettingsCard title="Effects & Blur" icon="pi pi-eye">
            <template #actions>
                 <div class="toggle-wrapper" style="display: flex; align-items: center; gap: 8px;">
                     <span class="setting-label" style="margin-bottom:0;">Blur</span>
                     <label class="switch">
                         <input type="checkbox" v-model="hyprlandConfig.blur_enabled">
                         <span class="slider round"></span>
                     </label>
                     <button class="primary-btn small-btn" @click="applyHyprlandConfig">
                         <i class="pi pi-save"></i> Apply
                     </button>
                 </div>
            </template>

            <div class="settings-grid" :class="{ 'disabled-grid': !hyprlandConfig.blur_enabled }" style="padding: 20px;">
                <div class="setting-control">
                    <label class="setting-label">Blur Size ({{ hyprlandConfig.blur_size }})</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.blur_size" min="1" max="20" :disabled="!hyprlandConfig.blur_enabled">
                </div>
                <div class="setting-control">
                    <label class="setting-label">Blur Passes ({{ hyprlandConfig.blur_passes }})</label>
                    <input type="range" class="slider-input" v-model="hyprlandConfig.blur_passes" min="1" max="5" :disabled="!hyprlandConfig.blur_enabled">
                </div>
            </div>
             <div class="settings-row" style="margin-top: 20px; border-top: 1px solid var(--card-border); padding: 20px;">
                <div class="setting-control" style="flex-direction: row; justify-content: space-between; align-items: center; flex:1;">
                    <label class="setting-label" style="margin: 0; margin-right: 12px;">Disable Hyprland Logo</label>
                    <label class="switch">
                        <input type="checkbox" v-model="hyprlandConfig.disable_logo">
                        <span class="slider round"></span>
                    </label>
                </div>
            </div>
        </SettingsCard>
    </PageLayout>
</template>

<style scoped>
/* View specific styles */
.card-content-wrapper {
    display: flex;
    align-items: center;
    gap: 20px;
    width: 100%;
}

.image-frame {
    width: 100%;
    aspect-ratio: 16/9;
    border-radius: 8px;
    overflow: hidden;
    background: var(--bg-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 8px solid rgb(102, 102, 102);
    box-shadow: 0 2px 8px rgba(0,0,0,0.02);
    position: relative;
    margin-top: 20px;
}

.wallpaper-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.change-btn-container {
    position: absolute;
    bottom: 10px;
    left: 0px;
    padding: 0 10px;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    width: 100%;
}

.btn-change {
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: white;
    padding: 10px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 8px;
    z-index: 10;
}

.btn-change:hover {
    background: rgba(0, 0, 0, 0.8);
    transform: translateY(-2px);
    border-color: rgba(255, 255, 255, 0.4);
}

.path-container {
    margin-top: 4px;
}

.path-highlight {
    display: inline-block;
    padding: 8px 12px;
    background-color: rgba(0, 0, 0, 0.6);
    border-radius: 6px;
    font-family: monospace;
    color: white;
    font-size: 0.85rem;
    word-break: break-all;
}

/* Taskbar Position Card */
.position-buttons {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
}

.position-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 16px 12px;
    background-color: var(--bg-secondary);
    border: 2px solid var(--card-border);
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s;
    color: var(--text-primary);
    font-size: 0.9rem;
    font-weight: 500;
}

.position-btn i {
    font-size: 1.2rem;
    color: var(--text-secondary);
    transition: color 0.2s;
}

.position-btn:hover:not(:disabled) {
    background-color: var(--card-bg);
    border-color: var(--accent-color);
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.position-btn.active {
    background-color: var(--accent-color);
    border-color: var(--accent-color);
    color: white;
}

.position-btn.active i {
    color: white;
}

/* Custom Select - Override or specific styling */
.custom-select {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: var(--bg-secondary);
    border: 1px solid var(--input-border, rgba(255,255,255,0.1));
    color: var(--text-primary);
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 1rem;
    width: 100%;
    cursor: pointer;
    transition: all 0.2s;
    background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: right 10px center;
    background-size: 16px;
    padding-right: 40px;
}

.custom-select:hover {
    border-color: var(--accent-color);
}

.custom-select:focus {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(var(--accent-color-rgb), 0.2);
}

.custom-select option {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
}

@media (max-width: 600px) {
    .position-buttons {
        grid-template-columns: repeat(2, 1fr);
    }
}
</style>
