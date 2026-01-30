<script setup lang="ts">
import logo from "../assets/logo.jpg";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();

const menuGroups = [
    {
        title: 'Network',
        items: [
            { label: 'Wi-Fi', icon: 'pi pi-wifi', path: '/wifi', color: '#525252' },
            { label: 'VPN', icon: 'pi pi-cloud', path: '/vpn', color: '#525252' },
            { label: 'Bluetooth', icon: 'pi pi-mobile', path: '/bluetooth', color: '#525252' },
        ]
    },
    {
        title: 'Personalization',
        items: [
            { label: 'Displays', icon: 'pi pi-desktop', path: '/displays', color: '#525252' },
            { label: 'Appearance', icon: 'pi pi-palette', path: '/appearance', color: '#525252' },
        ]
    },
    {
        title: 'Apps',
        items: [
            { label: 'Startup Apps', icon: 'pi pi-objects-column', path: '/startup', color: '#525252' },
        ]
    },
    {
        title: 'System',
        items: [
            { label: 'Shortcuts', icon: 'pi pi-address-book', path: '/shortcuts', color: '#525252' },
            { label: 'Environment', icon: 'pi pi-box', path: '/env', color: '#525252' },
            { label: 'System Update', icon: 'pi pi-history', path: '/system-update', color: '#525252' },
        ]
    },
];

const navigate = (path: string) => {
    router.push(path);
};

const isActive = (path: string) => route.path === path;

</script>

<template>
    <div class="sidebar-container">
        <div class="user-profile" @click="navigate('/about')" :class="{ 'active': isActive('/about') }">
            <div class="avatar">
                <img :src="logo" alt="User">
            </div>
            <div class="user-info">
                <div class="user-name">I4104</div>
                <div class="user-subtitle">System Settings</div>
            </div>
        </div>

        <hr class="separator">

        <!-- Menu List -->
        <div class="menu-list">
            <div v-for="(group, gIndex) in menuGroups" :key="gIndex" class="menu-group">
                <div v-if="group.title" class="menu-title">{{ group.title }}</div>
                <div 
                    v-for="(item, iIndex) in group.items" 
                    :key="iIndex" 
                    class="menu-item"
                    :class="{ 'active': isActive(item.path) }"
                    @click="navigate(item.path)"
                >
                    <div class="icon-wrapper" :style="{ 
                        backgroundColor: item.color, 
                        opacity: isActive(item.path) ? 1 : 0.5 
                    }">
                        <i :class="item.icon" style="color: white; font-size: 0.8rem;"></i>
                    </div>
                    <span class="label">{{ item.label }}</span>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.menu-title {
    font-size: 12px;
    font-weight: 400;
    color: var(--text-secondary);
    margin-left: 10px;
    margin-bottom: 5px;
}

.sidebar-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0;
    user-select: none;
}

.window-controls {
    display: flex;
    gap: 8px;
    padding: 18px 20px;
}
.control {
    width: 12px;
    height: 12px;
    border-radius: 50%;
}

.separator {
    margin: 0 10px;
    margin-bottom: 20px;
    border: none;
    border-top: 1px solid #686868;
}

.user-profile {
    display: flex;
    align-items: center;
    padding: 8px 16px;
    margin: 0 10px 10px;
    border-radius: 8px;
    cursor: pointer;
}
.user-profile:hover {
    background-color: var(--item-hover-bg);
}
.avatar img {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    margin-right: 12px;
    object-fit: cover;
    border: 1px solid #9e9e9e;
}
.user-name {
    font-weight: 700;
    font-size: 15px;
    margin-bottom: 2px;
}
.user-subtitle {
    font-size: 12px;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.menu-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 10px 10px;
}
.menu-group {
    margin-bottom: 12px;
}
.menu-item {
    display: flex;
    align-items: center;
    padding: 6px 10px;
    margin-bottom: 2px;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.1s;
}
.menu-item:hover {
    background-color: var(--item-hover-bg);
}

.menu-item.active {
    background-color: var(--item-active-bg);
    color: var(--item-active-text);
}

.icon-wrapper {
    width: 25px;
    height: 25px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 10px;
    opacity: 0.6;
}
.label {
    font-size: 15px;
    font-weight: 500;
}

</style>
