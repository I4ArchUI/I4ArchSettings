<script setup lang="ts">
import { useAppearanceViewModel } from '../viewmodels/appearance.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';

const {
    loading,
    cursorThemes,
    gtkThemes,
    selectedCursorTheme,
    selectedCursorSize,
    selectedGtkTheme,
    cursorSizes,
    hyprlandConfig,
    applyAppearanceSettings,
    applyHyprlandConfig
} = useAppearanceViewModel();
</script>

<template>
    <PageLayout>
        <template #title>
            <div class="view-title-container">
                <i class="pi pi-sliders-h title-icon"></i>
                <span>Themes</span>
            </div>
        </template>

        <div class="themes-container">
            <!-- Section 1: System Look & Feel (GTK + Cursor) -->
            <div class="settings-card glass-panel">
                <div class="card-header">
                    <div class="info-icon-wrapper">
                        <i class="pi pi-palette"></i>
                    </div>
                    <div class="info-content-text">
                        <h3>System Theme</h3>
                        <p>Customize GTK themes, mouse cursors, and cursor sizes</p>
                    </div>
                </div>
                
                <div class="card-body">
                    <div class="settings-grid">
                        <div class="setting-control">
                            <label class="setting-label">GTK Theme</label>
                            <div class="select-wrapper">
                                <select v-model="selectedGtkTheme" class="styled-select">
                                    <option value="" disabled>Select a theme...</option>
                                    <option 
                                        v-for="theme in gtkThemes" 
                                        :key="theme.name" 
                                        :value="theme.name"
                                    >
                                        {{ theme.name }}
                                    </option>
                                    <option v-if="gtkThemes.length === 0" value="Adwaita">Adwaita (Default)</option>
                                </select>
                                <i class="pi pi-chevron-down select-icon"></i>
                            </div>
                        </div>

                        <div class="setting-control">
                            <label class="setting-label">Cursor Theme</label>
                            <div class="select-wrapper">
                                <select v-model="selectedCursorTheme" class="styled-select">
                                    <option value="" disabled>Select a cursor theme...</option>
                                    <option 
                                        v-for="cursor in cursorThemes" 
                                        :key="cursor.name" 
                                        :value="cursor.name"
                                    >
                                        {{ cursor.name }}
                                    </option>
                                    <option v-if="cursorThemes.length === 0" value="Adwaita">Adwaita (Default)</option>
                                </select>
                                <i class="pi pi-chevron-down select-icon"></i>
                            </div>
                        </div>

                        <div class="setting-control">
                            <label class="setting-label">Cursor Size</label>
                            <div class="select-wrapper">
                                <select v-model.number="selectedCursorSize" class="styled-select">
                                    <option 
                                        v-for="size in cursorSizes" 
                                        :key="size" 
                                        :value="size"
                                    >
                                        {{ size }} px
                                    </option>
                                </select>
                                <i class="pi pi-chevron-down select-icon"></i>
                            </div>
                        </div>
                    </div>
                    
                    <div class="action-footer">
                        <button class="primary-btn" @click="applyAppearanceSettings" :disabled="loading">
                            <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                            <i v-else class="pi pi-check"></i>
                            Apply Themes
                        </button>
                    </div>
                </div>
            </div>

            <!-- Section 2: Hyprland Window Settings -->
            <div class="settings-card glass-panel">
                <div class="card-header">
                    <div class="info-icon-wrapper" style="background: linear-gradient(135deg, var(--accent-color), #2196f3); box-shadow: 0 4px 12px rgba(var(--accent-rgb), 0.3);">
                        <i class="pi pi-desktop"></i>
                    </div>
                    <div class="info-content-text">
                        <h3>Hyprland Customizations</h3>
                        <p>Manage gaps, border size, rounding, window opacity, and blur settings</p>
                    </div>
                </div>
                
                <div class="card-body">
                    <div class="settings-grid">
                        <!-- Gaps In -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Inner Gaps</span>
                                <span class="slider-val-badge">{{ hyprlandConfig.gaps_in }} px</span>
                            </div>
                            <input 
                                type="range" 
                                min="0" 
                                max="30" 
                                step="1"
                                v-model.number="hyprlandConfig.gaps_in" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Gaps Out -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Outer Gaps</span>
                                <span class="slider-val-badge">{{ hyprlandConfig.gaps_out }} px</span>
                            </div>
                            <input 
                                type="range" 
                                min="0" 
                                max="30" 
                                step="1"
                                v-model.number="hyprlandConfig.gaps_out" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Border Size -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Border Size</span>
                                <span class="slider-val-badge">{{ hyprlandConfig.border_size }} px</span>
                            </div>
                            <input 
                                type="range" 
                                min="0" 
                                max="10" 
                                step="1"
                                v-model.number="hyprlandConfig.border_size" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Rounding -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Rounding</span>
                                <span class="slider-val-badge">{{ hyprlandConfig.rounding }} px</span>
                            </div>
                            <input 
                                type="range" 
                                min="0" 
                                max="30" 
                                step="1"
                                v-model.number="hyprlandConfig.rounding" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Active Opacity -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Active Opacity</span>
                                <span class="slider-val-badge">{{ Math.round(hyprlandConfig.active_opacity * 100) }}%</span>
                            </div>
                            <input 
                                type="range" 
                                min="0.1" 
                                max="1.0" 
                                step="0.05"
                                v-model.number="hyprlandConfig.active_opacity" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Inactive Opacity -->
                        <div class="setting-control">
                            <div class="slider-label-row">
                                <span class="setting-label">Inactive Opacity</span>
                                <span class="slider-val-badge">{{ Math.round(hyprlandConfig.inactive_opacity * 100) }}%</span>
                            </div>
                            <input 
                                type="range" 
                                min="0.1" 
                                max="1.0" 
                                step="0.05"
                                v-model.number="hyprlandConfig.inactive_opacity" 
                                class="slider-input"
                            />
                        </div>

                        <!-- Blur Enabled Toggle -->
                        <div class="setting-control blur-toggle-container">
                            <div class="toggle-left">
                                <span class="setting-label text-primary-label">Enable Blur Effect</span>
                                <span class="toggle-desc">Toggle background window blur in Hyprland</span>
                            </div>
                            <label class="switch">
                                <input type="checkbox" v-model="hyprlandConfig.blur_enabled" />
                                <span class="slider round"></span>
                            </label>
                        </div>
                    </div>

                    <div class="action-footer">
                        <button class="primary-btn" @click="applyHyprlandConfig" :disabled="loading">
                            <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                            <i v-else class="pi pi-save"></i>
                            Apply Window Settings
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </PageLayout>
</template>

<style scoped>
.view-title-container {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary, #f5f5f7);
}

.title-icon {
    color: var(--accent-color, #e5c197);
    font-size: 22px;
}

.themes-container {
    display: flex;
    flex-direction: column;
    gap: 24px;
    width: 100%;
}

.card-body {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 24px;
}

/* Custom styling helper for slider values */
.slider-label-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2px;
}

.slider-val-badge {
    font-size: 12px;
    font-weight: 700;
    color: var(--accent-color, #e5c197);
    background: rgba(var(--accent-rgb, 229, 193, 151), 0.1);
    padding: 2px 8px;
    border-radius: 12px;
}

/* Custom layout for blur toggle switch row */
.blur-toggle-container {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    background: rgba(255, 255, 255, 0.02);
    padding: 14px 18px;
    border-radius: 10px;
    border: 1px solid var(--card-border);
    grid-column: span 2;
}

@media (max-width: 768px) {
    .blur-toggle-container {
        grid-column: span 1;
    }
}

.toggle-left {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.text-primary-label {
    text-transform: none;
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
}

.toggle-desc {
    font-size: 12px;
    color: var(--text-secondary, #a0a0a5);
}

/* Footer alignment */
.action-footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 8px;
    border-top: 1px solid var(--card-border);
    padding-top: 16px;
}
</style>
