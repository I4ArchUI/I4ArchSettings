<script setup lang="ts">
import { useAppsViewModel } from '../viewmodels/apps.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';
import LoadingState from '@/components/LoadingState.vue';

const {
    loading,
    searchQuery,
    activeTab,
    filteredApps,
    filteredPackages,
    switchTab,
    uninstallApp,
    uninstallPackage
} = useAppsViewModel();

</script>

<template>
    <PageLayout>
        <template #title>
             <div class="header-content">
                <span>Application Manager</span>
                <div class="tabs">
                    <button 
                        class="tab-btn" 
                        :class="{ active: activeTab === 'apps' }"
                        @click="switchTab('apps')"
                    >Apps</button>
                    <button 
                        class="tab-btn" 
                        :class="{ active: activeTab === 'packages' }"
                        @click="switchTab('packages')"
                    >Packages</button>
                </div>
            </div>
        </template>
        
        <template #actions>
            <div class="search-box">
                <i class="pi pi-search search-icon"></i>
                <input 
                    type="text" 
                    v-model="searchQuery" 
                    placeholder="Search..."
                    class="search-input"
                >
            </div>
        </template>

        <div class="content">
            <LoadingState v-if="loading && ((activeTab === 'apps' && filteredApps.length === 0) || (activeTab === 'packages' && filteredPackages.length === 0))" loading-text="Loading..." />

            <div v-else class="list-container">
                <!-- APPS LIST -->
                <div v-if="activeTab === 'apps'" class="apps-list">
                    <SettingsCard v-if="filteredApps.length === 0">
                        <div class="empty-state">No apps found</div>
                    </SettingsCard>
                    
                    <div v-else class="settings-card glass-panel" style="padding: 0;">
                        <div class="settings-group-list">
                            <div v-for="app in filteredApps" :key="app.desktop_file" class="settings-item">
                                <div class="item-icon">
                                    <i class="pi pi-box"></i> 
                                </div>
                                <div class="item-details">
                                    <span class="item-label">{{ app.name }}</span>
                                    <span class="item-sublabel">{{ app.description }}</span>
                                </div>
                                <button class="uninstall-btn" @click="uninstallApp(app)" title="Uninstall">
                                    <i class="pi pi-times"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- PACKAGES LIST -->
                <div v-if="activeTab === 'packages'" class="packages-list">
                    <SettingsCard v-if="filteredPackages.length === 0">
                        <div class="empty-state">No packages found</div>
                    </SettingsCard>

                    <div v-else class="settings-card glass-panel" style="padding: 0;">
                        <div class="settings-group-list">
                            <div v-for="pkg in filteredPackages.slice(0, 100)" :key="pkg.name" class="settings-item compact">
                                <div class="item-details">
                                    <span class="item-label mono">{{ pkg.name }}</span>
                                </div>
                                <div class="pkg-version">{{ pkg.version }}</div>
                                <button class="uninstall-btn" @click="uninstallPackage(pkg.name)" title="Uninstall">
                                    <i class="pi pi-times"></i>
                                </button>
                            </div>
                            <div v-if="filteredPackages.length > 100" class="more-items">
                                + {{ filteredPackages.length - 100 }} more (refine search)
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </PageLayout>
</template>

<style scoped>
.header-content {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.tabs {
    display: flex;
    gap: 10px;
}

.tab-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    padding: 4px 0;
    position: relative;
    transition: color 0.2s;
}

.tab-btn.active {
    color: var(--accent-color);
}

.tab-btn.active::after {
    content: '';
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 100%;
    height: 2px;
    background-color: var(--accent-color);
    border-radius: 2px;
}

.search-box {
    position: relative;
    width: 200px;
}

.search-input {
    width: 100%;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--card-border);
    border-radius: 8px;
    padding: 8px 12px 8px 32px;
    color: var(--text-primary);
    font-size: 13px;
    outline: none;
    transition: border-color 0.2s;
}

.search-input:focus {
    border-color: var(--accent-color);
}

.search-icon {
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-secondary);
    font-size: 12px;
    pointer-events: none;
}

.content {
    display: flex;
    flex-direction: column;
}

.settings-group-list {
    display: flex;
    flex-direction: column;
}

.settings-item {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--card-border);
    transition: background-color 0.1s;
}
.settings-item:last-child {
    border-bottom: none;
}
.settings-item:hover {
    background-color: var(--item-hover-bg);
}

.settings-item.compact {
    padding: 8px 16px;
}

.item-icon {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    background-color: var(--bg-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 14px;
    color: var(--text-secondary);
    font-size: 1.2rem;
}

.item-details {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
}

.item-label {
    font-weight: 500;
    font-size: 0.95rem;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
.item-label.mono {
    font-family: monospace;
    font-size: 0.9rem;
}

.item-sublabel {
    font-size: 0.8rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.pkg-version {
    color: var(--text-secondary);
    font-size: 12px;
    margin-right: 15px;
    font-family: monospace;
}

.uninstall-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
}

.uninstall-btn:hover {
    background-color: rgba(255, 80, 80, 0.15);
    color: #ff5050;
}

.more-items {
    padding: 10px;
    text-align: center;
    font-size: 12px;
    color: var(--text-secondary);
    border-top: 1px solid var(--card-border);
}

.empty-state {
    padding: 30px;
    text-align: center;
    color: var(--text-secondary);
}
</style>
