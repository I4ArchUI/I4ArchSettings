<script setup lang="ts">


const group1 = [
    { label: 'About', icon: 'pi pi-info-circle' },
    { label: 'Software Update', icon: 'pi pi-cog' }, // spin icon usually
    { label: 'Storage', icon: 'pi pi-database' },
];

const group2 = [
    { label: 'AirDrop & Handoff', icon: 'pi pi-send' },
    { label: 'Login Items', icon: 'pi pi-user-plus' },
];

const group3 = [
    { label: 'Language & Region', icon: 'pi pi-globe' },
    { label: 'Date & Time', icon: 'pi pi-calendar' },
];

const group4 = [
    { label: 'Sharing', icon: 'pi pi-share-alt' },
    { label: 'Time Machine', icon: 'pi pi-clock' },
    { label: 'Transfer or Reset', icon: 'pi pi-refresh' },
    { label: 'Startup Disk', icon: 'pi pi-save' },
];
</script>

<template>
  <div class="general-view">
    <div class="header">
        <h1 class="page-title">General</h1>
    </div>

    <div class="settings-content">
        <!-- Group 1 -->
        <div class="settings-group">
            <div class="settings-item" v-for="(item, index) in group1" :key="index">
                <div class="item-icon">
                    <i :class="item.icon"></i>
                </div>
                <div class="item-details">
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <i class="pi pi-chevron-right chevron"></i>
            </div>
        </div>

        <!-- Group 2 -->
        <div class="settings-group">
            <div class="settings-item" v-for="(item, index) in group2" :key="index">
                <div class="item-icon">
                    <i :class="item.icon"></i>
                </div>
                <div class="item-details">
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <i class="pi pi-chevron-right chevron"></i>
            </div>
        </div>

        <!-- Group 3 -->
        <div class="settings-group">
            <div class="settings-item" v-for="(item, index) in group3" :key="index">
                <div class="item-icon">
                    <i :class="item.icon"></i>
                </div>
                <div class="item-details">
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <i class="pi pi-chevron-right chevron"></i>
            </div>
        </div>

        <!-- Group 4 -->
        <div class="settings-group">
             <div class="settings-item" v-for="(item, index) in group4" :key="index">
                <div class="item-icon">
                    <i :class="item.icon"></i>
                </div>
                <div class="item-details">
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <i class="pi pi-chevron-right chevron"></i>
            </div>
        </div>
    </div>
  </div>
</template>

<style scoped>
.general-view {
    padding: 0 40px 40px 40px;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
}

.header {
    padding: 20px 0 16px 0;
    position: sticky;
    top: 0;
    z-index: 10;
    background-color: var(--content-bg);
}

.page-title {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
}

.settings-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.settings-group {
    background-color: #fff;
    border-radius: 10px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05); /* very subtle */
    border: 1px solid rgba(0,0,0,0.05);
    overflow: hidden;
}

.settings-item {
    display: flex;
    align-items: center;
    padding: 10px 16px;
    min-height: 48px;
    cursor: pointer;
    background-color: #fff;
    transition: background-color 0.1s;
    position: relative;
}

.settings-item:not(:last-child)::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50px; /* offset icon width */
    right: 0;
    height: 1px;
    background-color: var(--separator-color);
}

.settings-item:hover {
    background-color: var(--item-hover-bg);
}

.item-icon {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background-color: #e5e5e5;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 14px;
    color: #666;
}

/* Specific styling for item icons if needed */
.item-icon i {
    font-size: 16px;
}

.item-details {
    flex: 1;
}

.item-label {
    font-size: 13px;
    font-weight: 500;
}

.chevron {
    font-size: 12px;
    color: #c7c7cc;
    font-weight: bold;
}

@media (prefers-color-scheme: dark) {
    .settings-group {
        background-color: rgba(255,255,255,0.05);
        border: 1px solid rgba(255,255,255,0.05);
    }
    .settings-item {
        background-color: transparent;
    }
    .item-icon {
        background-color: rgba(255,255,255,0.1);
        color: #ddd;
    }
    .chevron {
        color: #555;
    }
}
</style>
