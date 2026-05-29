<script setup lang="ts">
import AppSidebar from '../components/AppSidebar.vue';
import logo from '../assets/logo.jpg';
import { useSearch } from '../composables/useSearch';

const { searchQuery } = useSearch();
</script>

<template>
    <div class="main-window glass-frame">
        <!-- Unified Header -->
        <header class="window-header">
            <div class="header-left">
                <div class="user-avatar">
                    <img :src="logo" alt="User Avatar" />
                </div>
                <div class="header-title-info">
                    <span class="header-title">Settings</span>
                    <span class="header-subtitle">i4arch system settings</span>
                </div>
            </div>
            
            <div class="header-center">
                <div class="search-pill">
                    <i class="pi pi-search search-icon"></i>
                    <input 
                        type="text" 
                        v-model="searchQuery" 
                        placeholder="Search settings... (Ctrl+/)" 
                        class="search-input"
                    />
                </div>
            </div>
            
            <div class="header-right"></div>
        </header>

        <!-- Body Layout -->
        <div class="window-body">
            <aside class="sidebar-pane">
                <AppSidebar />
            </aside>
            <main class="content-pane">
                <slot></slot>
            </main>
        </div>
    </div>
</template>

<style scoped>
.main-window {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    border-radius: var(--window-border-radius);
    overflow: hidden;
    background: var(--content-bg);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
    border: 1px solid var(--card-border);
}

.glass-frame {
    backdrop-filter: var(--glass-blur, blur(30px) saturate(140%));
}

/* Header Styles */
.window-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 64px;
    padding: 0 20px;
    background: rgba(15, 15, 20, 0.4);
    border-bottom: 1px solid var(--separator-color);
    user-select: none;
    z-index: 100;
}

.header-left {
    display: flex;
    align-items: center;
    gap: 12px;
}

.user-avatar img {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    object-fit: cover;
    border: 1px solid rgba(255, 255, 255, 0.2);
}

.header-title-info {
    display: flex;
    flex-direction: column;
}

.header-title {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
}

.header-subtitle {
    font-size: 11px;
    color: var(--text-secondary);
}

/* Center Search Pill */
.header-center {
    flex: 1;
    max-width: 320px;
    margin: 0 20px;
}

.search-pill {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
    height: 36px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 18px;
    padding: 0 14px;
    transition: all 0.2s ease;
}

.search-pill:focus-within {
    background: rgba(255, 255, 255, 0.08);
    border-color: var(--accent-color);
    box-shadow: 0 0 10px rgba(var(--accent-rgb), 0.15);
}

.search-icon {
    font-size: 12px;
    color: var(--text-secondary);
    margin-right: 8px;
}

.search-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 13px;
    width: 100%;
}

.search-input::placeholder {
    color: rgba(255, 255, 255, 0.35);
}

/* Right Window Controls */
.header-right {
    display: flex;
    align-items: center;
}

.window-controls-glass {
    display: flex;
    gap: 4px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 2px;
}

.control-btn {
    width: 28px;
    height: 28px;
    border-radius: 8px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 12px;
}

.control-btn:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.08);
}

.close-btn:hover {
    color: #ff5f56;
    background: rgba(255, 95, 86, 0.15);
}

/* Body Layout */
.window-body {
    display: flex;
    flex: 1;
    overflow: hidden;
    height: calc(100% - 64px);
}

.sidebar-pane {
    width: var(--sidebar-width);
    background-color: var(--sidebar-bg);
    border-right: 1px solid var(--separator-color);
    padding: 12px 2px;
    display: flex;
    flex-direction: column;
}

.content-pane {
    flex: 1;
    background-color: var(--content-bg);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
}
</style>
