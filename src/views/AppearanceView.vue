<script setup lang="ts">
import { useAppearanceViewModel } from '../viewmodels/appearance.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';

const {
    loading,
    isDark,
    currentWallpaperSrc,
    toggleTheme,
    pickWallpaper,
    handleImageError,
    wallpapers,
    transparency,
    getWallpaperUrl,
    selectWallpaper
} = useAppearanceViewModel();
</script>

<template>
    <PageLayout>
        <!-- Page Title -->
        <template #title>
            <div class="view-title-container">
                <i class="pi pi-palette title-icon"></i>
                <span>Wallpaper & Colors</span>
            </div>
        </template>

        <!-- Main Dashboard Card -->
        <div class="dashboard-glass-panel">
            <!-- Top Configuration Row (Three Columns) -->
            <div class="config-columns">
                <!-- Column 1: Wallpaper Preview -->
                <div class="config-col preview-col">
                    <div class="wallpaper-preview-frame">
                        <img 
                            :src="currentWallpaperSrc" 
                            alt="Current Wallpaper Preview" 
                            class="preview-img"
                            @error="handleImageError"
                        />
                    </div>
                </div>

                <!-- Column 2: Choose File Button -->
                <div class="config-col select-file-col" @click="pickWallpaper" :class="{ 'disabled': loading }">
                    <div class="choose-file-card">
                        <i v-if="loading" class="pi pi-spin pi-spinner choose-icon"></i>
                        <i v-else class="pi pi-images choose-icon"></i>
                        <span class="choose-label">Choose File</span>
                    </div>
                </div>

                <!-- Column 3: Light/Dark Selector -->
                <div class="config-col theme-selector-col">
                    <div class="theme-cards-container">
                        <!-- Light Theme Card -->
                        <div 
                            class="theme-card light-card" 
                            :class="{ 'active': !isDark }"
                            @click="!isDark ? null : toggleTheme()"
                        >
                            <i class="pi pi-sun theme-icon"></i>
                            <span class="theme-label">Light</span>
                        </div>

                        <!-- Dark Theme Card -->
                        <div 
                            class="theme-card dark-card" 
                            :class="{ 'active': isDark }"
                            @click="isDark ? null : toggleTheme()"
                        >
                            <i class="pi pi-moon theme-icon"></i>
                            <span class="theme-label">Dark</span>
                        </div>
                    </div>
                </div>
            </div>


            <!-- Transparency Control Row -->
            <div class="settings-row-item transparency-row">
                <div class="row-left">
                    <i class="pi pi-eye row-icon"></i>
                    <div class="row-text-info">
                        <span class="row-label">Transparency</span>
                    </div>
                </div>
                <div class="row-right">
                    <label class="switch-toggle">
                        <input type="checkbox" v-model="transparency" />
                        <span class="slider-round"></span>
                    </label>
                </div>
            </div>

            <hr class="panel-divider" />

            <!-- Quick Select Section -->
            <div class="quick-select-header">
                <div class="quick-left">
                    <span class="quick-title">Quick select</span>
                </div>
            </div>

            <!-- Wallpapers Dynamic Grid -->
            <div class="wallpaper-grid-container">
                <div v-if="loading" style="display: flex; justify-content: center; align-items: center; min-height: 120px;">
                    <i class="pi pi-spin pi-spinner" style="font-size: 24px; color: var(--accent-color);"></i>
                </div>
                <div v-else-if="wallpapers.length === 0" class="empty-wallpapers-message">
                    <i class="pi pi-exclamation-triangle warning-icon"></i>
                    <span>Failed to load wallpapers from online source.</span>
                </div>
                <div v-else class="wallpapers-grid">
                    <div 
                        v-for="(wall, idx) in wallpapers" 
                        :key="idx" 
                        class="wallpaper-grid-card"
                        :class="{ 'active': currentWallpaperSrc.includes(wall) || (idx === 1 && currentWallpaperSrc.startsWith('data:image')) }"
                        @click="selectWallpaper(wall)"
                    >
                        <img 
                            :src="getWallpaperUrl(wall)" 
                            alt="Wallpaper Thumbnail" 
                            class="grid-thumbnail-img"
                            loading="lazy"
                        />
                        <div class="grid-card-active-overlay">
                            <i class="pi pi-check-circle check-badge"></i>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </PageLayout>
</template>

<style scoped>
/* Page Layout Wrapper overrides if any */
.view-title-container {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
}

.title-icon {
    color: var(--accent-color);
    font-size: 22px;
}

/* Dashboard Main Card styling */
.dashboard-glass-panel {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 20px;
    padding: 24px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(20px);
    display: flex;
    flex-direction: column;
    gap: 20px;
}

/* Three Columns Configuration layout */
.config-columns {
    display: grid;
    grid-template-columns: 1.4fr 1fr 1fr;
    gap: 16px;
    width: 100%;
}

.config-col {
    border-radius: 14px;
    overflow: hidden;
    height: 120px;
}

/* Preview Column (Landscape) */
.wallpaper-preview-frame {
    width: 100%;
    height: 100%;
    border-radius: 14px;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
}

.preview-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

/* Select File Column */
.select-file-col {
    background: rgba(255, 255, 255, 0.03);
    border: 1px dashed rgba(255, 255, 255, 0.15);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
}

.select-file-col:hover:not(.disabled) {
    background: rgba(255, 255, 255, 0.06);
    border-color: var(--accent-color);
}

.choose-file-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: var(--text-secondary);
}

.select-file-col:hover .choose-file-card {
    color: var(--text-primary);
}

.choose-icon {
    font-size: 24px;
    opacity: 0.8;
}

.choose-label {
    font-size: 13px;
    font-weight: 500;
}

/* Theme Selector Cards */
.theme-selector-col {
    display: flex;
    background: transparent;
}

.theme-cards-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    width: 100%;
    height: 100%;
}

.theme-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-card:hover {
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-primary);
}

.theme-card .theme-icon {
    font-size: 20px;
}

.theme-card .theme-label {
    font-size: 13px;
    font-weight: 500;
}

/* Light Active theme card style */
.light-card.active {
    background: #ffffff;
    color: #121214;
    border-color: #ffffff;
    box-shadow: 0 4px 15px rgba(255, 255, 255, 0.15);
}

/* Dark Active theme card style (Peach/Amber theme) */
.dark-card.active {
    background: var(--accent-color);
    color: #121214;
    border-color: var(--accent-color);
    box-shadow: 0 4px 15px rgba(229, 193, 151, 0.25);
}

/* Dynamic color scheme pills styling */
.schemes-wrapper {
    width: 100%;
    overflow-x: auto;
    padding-bottom: 4px;
}

.schemes-pills {
    display: flex;
    gap: 8px;
    flex-wrap: nowrap;
}

.scheme-pill-btn {
    padding: 8px 16px;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.2s ease;
}

.scheme-pill-btn:hover {
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-primary);
}

.scheme-pill-btn.active {
    background: var(--accent-color);
    color: #121214;
    border-color: var(--accent-color);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(229, 193, 151, 0.2);
}

/* Transparency Row styling */
.settings-row-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
}

.row-left {
    display: flex;
    align-items: center;
    gap: 12px;
}

.row-icon {
    font-size: 16px;
    color: var(--text-secondary);
}

.row-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
}

/* Toggle Switch customized style */
.switch-toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
}

.switch-toggle input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider-round {
    position: absolute;
    cursor: pointer;
    top: 0; left: 0; right: 0; bottom: 0;
    background-color: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.08);
    transition: .3s;
    border-radius: 24px;
}

.slider-round:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: #ffffff;
    transition: .3s;
    border-radius: 50%;
}

.switch-toggle input:checked + .slider-round {
    background-color: var(--accent-color);
    border-color: var(--accent-color);
}

.switch-toggle input:checked + .slider-round:before {
    transform: translateX(20px);
    background-color: #121214;
}

/* Divider styling */
.panel-divider {
    border: none;
    border-top: 1px solid var(--separator-color);
    margin: 8px 0;
}

/* Quick Select styling */
.quick-select-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
}

.quick-left {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.quick-title {
    font-size: 15px;
    font-weight: 700;
    color: var(--text-primary);
}

.folder-path-display {
    font-size: 11px;
    color: var(--text-secondary);
}

.path-highlight {
    font-family: 'JetBrains Mono', monospace;
    background: rgba(255, 255, 255, 0.04);
    padding: 2px 6px;
    border-radius: 4px;
    color: var(--accent-color);
}

.quick-actions {
    display: flex;
    gap: 8px;
}

.action-pill-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.action-pill-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
}

.action-pill-btn i {
    font-size: 11px;
}

/* Wallpaper dynamic grid layout styling */
.wallpaper-grid-container {
    width: 100%;
    margin-top: 4px;
}

.wallpapers-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: 10px;
    width: 100%;
}

.wallpaper-grid-card {
    position: relative;
    aspect-ratio: 1;
    border-radius: 10px;
    overflow: hidden;
    border: 2px solid transparent;
    cursor: pointer;
    background: rgba(0, 0, 0, 0.2);
    transition: all 0.2s ease;
}

.wallpaper-grid-card:hover {
    transform: scale(1.03);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
}

.grid-thumbnail-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.grid-card-active-overlay {
    position: absolute;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.2s ease;
}

.check-badge {
    font-size: 18px;
    color: var(--accent-color);
}

/* Active Thumbnail grid item style */
.wallpaper-grid-card.active {
    border-color: var(--accent-color);
    box-shadow: 0 0 10px rgba(229, 193, 151, 0.4);
}

.wallpaper-grid-card.active .grid-card-active-overlay {
    opacity: 1;
}

@media (max-width: 900px) {
    .config-columns {
        grid-template-columns: 1fr;
    }
    
    .wallpapers-grid {
        grid-template-columns: repeat(4, 1fr);
    }
}

@media (max-width: 600px) {
    .wallpapers-grid {
        grid-template-columns: repeat(3, 1fr);
    }
    .quick-select-header {
        flex-direction: column;
        align-items: flex-start;
    }
}

.empty-wallpapers-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 40px 20px;
    background: rgba(255, 95, 86, 0.05);
    border: 1px dashed rgba(255, 95, 86, 0.15);
    border-radius: 12px;
    color: #ff5f56;
    font-size: 13px;
    font-weight: 500;
    text-align: center;
    width: 100%;
}

.warning-icon {
    font-size: 24px;
}
</style>
