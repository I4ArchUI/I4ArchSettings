<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import { useSearch } from "../composables/useSearch";
import { computed } from "vue";

const router = useRouter();
const route = useRoute();
const { searchQuery } = useSearch();

const menuItems = [
    { label: 'About System', icon: 'pi pi-info-circle', path: '/about' },
    { label: 'Wi-Fi', icon: 'pi pi-wifi', path: '/wifi' },
    { label: 'VPN', icon: 'pi pi-cloud', path: '/vpn' },
    { label: 'Bluetooth', icon: 'pi pi-mobile', path: '/bluetooth' },
    { label: 'Wallpaper', icon: 'pi pi-palette', path: '/appearance' },
    { label: 'Themes', icon: 'pi pi-sliders-h', path: '/themes' },
    { label: 'Displays', icon: 'pi pi-desktop', path: '/displays' },
    { label: 'Installed Apps', icon: 'pi pi-th-large', path: '/apps' },
    { label: 'Startup Apps', icon: 'pi pi-cog', path: '/startup' },
    { label: 'Keybinds', icon: 'pi pi-key', path: '/shortcuts' },
    { label: 'Environment', icon: 'pi pi-box', path: '/env' },
    { label: 'System Update', icon: 'pi pi-history', path: '/system-update' },
];

// Computed list filtered by the header search query
const filteredMenuItems = computed(() => {
    if (!searchQuery.value.trim()) return menuItems;
    const query = searchQuery.value.toLowerCase().trim();
    return menuItems.filter(item => item.label.toLowerCase().includes(query));
});

const navigate = (path: string) => {
    router.push(path);
};

const isActive = (item: any) => {
    return route.path === item.path;
};
</script>

<template>
    <div class="sidebar-container">
        <!-- Menu List -->
        <div class="menu-list">
            <div 
                v-for="(item, index) in filteredMenuItems" 
                :key="index" 
                class="menu-item"
                :class="{ 'active': isActive(item) }"
                @click="navigate(item.path)"
            >
                <div class="icon-wrapper">
                    <i :class="item.icon" class="item-icon"></i>
                </div>
                <span class="label">{{ item.label }}</span>
            </div>

            <!-- Empty Search State -->
            <div v-if="filteredMenuItems.length === 0" class="empty-search">
                <i class="pi pi-search"></i>
                <span>No match found</span>
            </div>
        </div>
    </div>
</template>

<style scoped>
.sidebar-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0;
    user-select: none;
}

.menu-list {
    flex: 1;
    overflow-y: auto;
    padding: 10px;
}

.menu-item {
    display: flex;
    align-items: center;
    padding: 10px 14px;
    margin-bottom: 4px;
    border-radius: 14px;
    cursor: pointer;
    transition: color 0.45s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                transform 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    color: var(--text-secondary);
    position: relative;
    overflow: hidden;
    z-index: 0;
    border: 1px solid transparent;
}

/* Glassmorphism sliding fill layer */
.menu-item::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.06);
    backdrop-filter: blur(10px);
    transform: translateX(-100%);
    transition: transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    z-index: -1;
}

.menu-item:hover::before {
    transform: translateX(0);
}

.menu-item:hover {
    color: var(--text-primary);
    transform: translateX(2px);
    border-color: rgba(255, 255, 255, 0.06);
}

/* Active: accent glass fill sweeps in from left */
.menu-item.active::before {
    background: rgba(229, 193, 151, 0.14);
    backdrop-filter: blur(16px);
    transform: translateX(0);
    animation: swipe-in 0.65s cubic-bezier(0.25, 0.46, 0.45, 0.94) both;
}

.menu-item.active {
    color: var(--item-active-text);
    font-weight: 600;
    border-color: rgba(229, 193, 151, 0.15);
    box-shadow: inset 3px 0 0 var(--accent-color);
}

@keyframes swipe-in {
    from { transform: translateX(-100%); }
    to   { transform: translateX(0); }
}

.icon-wrapper {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    flex-shrink: 0;
}

.item-icon {
    font-size: 14px;
    transition: transform 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.menu-item.active .item-icon {
    transform: scale(1.1);
    filter: drop-shadow(0 0 4px rgba(229, 193, 151, 0.5));
}

.label {
    font-size: 13px;
    font-weight: 500;
    letter-spacing: 0.01em;
}

/* Empty Search styles */
.empty-search {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 10px;
    color: var(--text-secondary);
    gap: 8px;
    font-size: 12px;
}

.empty-search i {
    font-size: 20px;
    opacity: 0.4;
}
</style>
