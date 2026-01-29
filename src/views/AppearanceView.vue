<script setup lang="ts">
import { useAppearanceViewModel } from '../viewmodels/appearance.viewmodel';

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
    applyAppearanceSettings
} = useAppearanceViewModel();
</script>

<template>
    <div class="wallpaper-view">
        <div class="header">
            <h1 class="page-title">Appearance</h1>
        </div>

        <div class="content-container">
            <div class="preview-card">
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
            </div>

            <!-- Theme Card -->
            <div class="theme-card">
                <div class="theme-content">
                    <div class="info-icon-wrapper">
                        <i :class="['pi', isDark ? 'pi-moon' : 'pi-sun']"></i>
                    </div>
                    <div class="info-content-text">
                        <h3 class="info-title">Dark Mode</h3>
                    </div>
                </div>
                <label class="switch">
                    <input type="checkbox" :checked="isDark" @change="toggleTheme">
                    <span class="slider round"></span>
                </label>
            </div>

            <!-- Cursor Settings -->
            <div class="settings-card">
                <div class="card-header">
                    <div class="info-icon-wrapper">
                        <i class="pi pi-stop-circle"></i>
                    </div>
                    <div class="info-content-text">
                        <h3 class="info-title">Cursor</h3>
                    </div>
                </div>
                <div class="settings-row">
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
            </div>

            <!-- GTK Theme Settings -->
            <div class="settings-card">
                <div class="card-header">
                    <div class="info-icon-wrapper">
                        <i class="pi pi-palette"></i>
                    </div>
                    <div class="info-content-text">
                        <h3 class="info-title">GTK & Shell Theme</h3>
                    </div>
                </div>
                <div class="settings-row">
                    <div class="setting-control" style="width: 100%;">
                        <select class="custom-select" v-model="selectedGtkTheme" @change="applyAppearanceSettings" style="width: 100%;">
                            <option v-for="theme in gtkThemes" :key="theme.name" :value="theme.name">{{ theme.name }}</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- Taskbar Position Card -->
            <div class="taskbar-card">
                <div class="taskbar-header">
                    <div class="info-icon-wrapper">
                        <i class="pi pi-window-maximize"></i>
                    </div>
                    <div class="info-content-text">
                        <h3 class="info-title">Taskbar Position</h3>
                    </div>
                </div>
                <div class="position-buttons">
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
            </div>
        </div>
    </div>
</template>

<style scoped>
.info-icon-wrapper i {
    font-size: 24px;
}

.wallpaper-view {
    padding: 0 40px 40px 40px;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
}

.header {
    padding: 20px 0 30px 0;
}

.page-title {
    font-size: 24px;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary);
}

.content-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.preview-card {
    box-shadow: 0 2px 8px rgba(0,0,0,0.02);
    display: flex;
    flex-direction: column;
    gap: 20px;
    align-items: center;
    justify-content: center;
    width: 100%;
    box-sizing: border-box;
}

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

.btn-change:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.path-container {
    display: flex;
    align-items: center;
    gap: 10px;
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

.theme-card {
    background-color: var(--card-bg);
    border-radius: 12px;
    padding: 15px 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 1px solid var(--card-border);
}

.theme-content {
    display: flex;
    align-items: center;
    gap: 16px;
}

/* Taskbar Position Card */
.taskbar-card {
    background-color: var(--card-bg);
    border-radius: 12px;
    padding: 15px 20px;
    border: 1px solid var(--card-border);
    transition: transform 0.2s, box-shadow 0.2s;
}

.taskbar-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
}

.taskbar-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;
}

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

.position-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

@media (max-width: 600px) {
    .position-buttons {
        grid-template-columns: repeat(2, 1fr);
    }
}


/* General Settings Card */
.settings-card {
    background-color: var(--card-bg);
    border-radius: 12px;
    padding: 20px;
    border: 1px solid var(--card-border);
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.card-header {
    display: flex;
    align-items: center;
    gap: 16px;
}

.settings-row {
    display: flex;
    gap: 20px;
    width: 100%;
}

.setting-control {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
}

.setting-label {
    font-size: 0.9rem;
    color: var(--text-secondary);
    font-weight: 500;
}

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

</style>
