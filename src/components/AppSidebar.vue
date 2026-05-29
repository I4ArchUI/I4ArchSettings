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
    { label: 'Wallpaper & Colors', icon: 'pi pi-palette', path: '/appearance' },
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
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    color: var(--text-secondary);
}

.menu-item:hover {
    background-color: var(--item-hover-bg);
    color: var(--text-primary);
    transform: translateX(2px);
}

.menu-item.active {
    background-color: var(--item-active-bg);
    color: var(--item-active-text);
    font-weight: 600;
    border-left: 3px solid var(--accent-color);
}

.icon-wrapper {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
}

.item-icon {
    font-size: 14px;
}

.label {
    font-size: 13px;
    font-weight: 500;
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
