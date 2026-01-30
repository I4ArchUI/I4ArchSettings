<script setup lang="ts">
import { useUpdatesViewModel } from '../viewmodels/updates.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import LoadingState from '@/components/LoadingState.vue';
import loadingGif from '@/assets/loading-cat.gif';

const {
    updates,
    loading,
    error,
    checkUpdates,
    updateSystem
} = useUpdatesViewModel();

</script>

<template>
    <PageLayout>
        <template #title>System Update</template>
        <template #actions>
            <button class="icon-button" @click="checkUpdates" title="Refresh">
                <i class="pi pi-refresh" v-if="!loading"></i>
                <img :src="loadingGif" style="width: 20px; height: 20px;" v-else>
            </button>
        </template>

        <div class="content">
            <div v-if="error" class="error-banner">
                <i class="pi pi-exclamation-triangle"></i>
                <span>{{ error }}</span>
            </div>

            <div v-if="loading && updates.length === 0" class="loading-container">
                 <LoadingState loading-text="Checking for updates..." />
            </div>

            <div v-else-if="updates.length === 0 && !loading && !error" class="empty-state">
                <i class="pi pi-check-circle" style="font-size: 4rem; color: var(--accent-color); margin-bottom: 1rem;"></i>
                <p>Your system is up to date</p>
            </div>

            <div v-else-if="updates.length > 0" class="updates-container">
                <div class="header-card">
                     <div class="update-count">
                        <span class="count">{{ updates.length }}</span>
                        <span class="label">Updates Available</span>
                     </div>
                     <button class="update-button" @click="updateSystem">
                        <i class="pi pi-download"></i>
                        Update All
                     </button>
                </div>

                <div class="settings-card glass-panel" style="padding: 0;">
                    <div class="settings-group-list">
                        <div v-for="pkg in updates" :key="pkg.name" class="settings-item">
                            <div class="item-icon">
                                <i class="pi pi-box"></i>
                            </div>
                            <div class="item-details">
                                <span class="item-label">{{ pkg.name }}</span>
                                <div class="version-row">
                                    <span class="version old">{{ pkg.old_version }}</span>
                                    <i class="pi pi-arrow-right arrow"></i>
                                    <span class="version new">{{ pkg.new_version }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </PageLayout>
</template>

<style scoped>
.content {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.icon-button {
    background: transparent;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    padding: 8px;
    border-radius: 50%;
    transition: background-color 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
}
.icon-button:hover {
    background-color: var(--item-hover-bg);
}

.spin {
    animation: spin 1s linear infinite;
}
@keyframes spin { 100% { transform: rotate(360deg); } }

.header-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    background-color: var(--card-bg); /* Fallback */
    background: linear-gradient(145deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border-radius: 12px;
    border: 1px solid var(--card-border);
}

.update-count {
    display: flex;
    flex-direction: column;
}
.count {
    font-size: 2rem;
    font-weight: 700;
    color: var(--text-primary);
}
.label {
    font-size: 0.9rem;
    color: var(--text-secondary);
}

.update-button {
    background-color: var(--accent-color);
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: opacity 0.2s;
}
.update-button:hover {
    opacity: 0.9;
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
}
.settings-item:last-child {
    border-bottom: none;
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
}

.item-details {
    display: flex;
    flex-direction: column;
    flex: 1;
}

.item-label {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text-primary);
}

.version-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-top: 2px;
}

.version.new {
    color: var(--accent-color); 
    font-weight: 500;
}

.arrow {
    font-size: 0.7rem;
    opacity: 0.5;
}

.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 0;
    color: var(--text-secondary);
}

.error-banner {
    background-color: rgba(255, 100, 100, 0.1);
    border: 1px solid rgba(255, 100, 100, 0.3);
    color: #ff6b6b;
    padding: 12px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    gap: 10px;
}

.updates-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

/* Glass panel style override or addition if not global */
.glass-panel {
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
}
</style>
