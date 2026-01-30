
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import logo from "../assets/logo.jpg";
import PageLayout from '../components/common/PageLayout.vue';

interface SystemInfo {
    hostname: string;
    os_name: string;
    kernel_version: string;
    cpu_model: string;
    memory_total: string;
    gpu_info: string;
    disk_total?: string;
    disk_used?: string;
    disk_percent?: number;
}

const loading = ref(true);
const sysInfo = ref<SystemInfo>({
    hostname: '',
    os_name: '',
    kernel_version: '',
    cpu_model: '',
    memory_total: '',
    gpu_info: '',
    disk_total: '',
    disk_used: '',
    disk_percent: 0
});

onMounted(async () => {
    try {
        sysInfo.value = await invoke('get_system_info');
    } catch (e) {
    } finally {
        loading.value = false;
    }
});
</script>

<template>
  <PageLayout>
    <template #title>About System</template>

    <div v-if="loading" class="loading-container">
        <i class="pi pi-spin pi-spinner" style="font-size: 2rem"></i>
        <p>Fetching system information...</p>
    </div>

    <div v-else class="about-content">
        <div class="hero-section glass-panel">
            <div class="os-logo">
                <img :src="logo" class="logo-img" :alt="sysInfo.os_name">
            </div>
            <div class="os-title">
                <h2>{{ sysInfo.hostname }}</h2>
                <div class="disk-usage" v-if="sysInfo.disk_total">
                    <div class="disk-info-row">
                         <span class="disk-label">{{ sysInfo.os_name || 'Linux System' }} - {{ sysInfo.kernel_version }}</span>
                         <span class="disk-stats">{{ sysInfo.disk_used }} / {{ sysInfo.disk_total }}</span>
                    </div>
                    <div class="progress-track" :title="sysInfo.disk_percent + '% used'">
                        <div class="progress-fill" :style="{ width: sysInfo.disk_percent + '%' }"></div>
                    </div>
                </div>
            </div>
        </div>

        <div class="info-grid">
            <div class="info-card glass-panel">
                <div class="card-icon"><i class="pi pi-cog"></i></div>
                <div class="card-content">
                    <span class="label">Kernel</span>
                    <span class="value">{{ sysInfo.kernel_version }}</span>
                </div>
            </div>

            <div class="info-card glass-panel">
                <div class="card-icon"><i class="pi pi-server"></i></div>
                <div class="card-content">
                    <span class="label">Processor</span>
                    <span class="value">{{ sysInfo.cpu_model }}</span>
                </div>
            </div>

            <div class="info-card glass-panel">
                <div class="card-icon"><i class="pi pi-database"></i></div>
                <div class="card-content">
                    <span class="label">Memory</span>
                    <span class="value">{{ sysInfo.memory_total }}</span>
                </div>
            </div>
            
            <div class="info-card glass-panel" v-if="sysInfo.gpu_info && sysInfo.gpu_info !== 'Unknown'">
                <div class="card-icon"><i class="pi pi-image"></i></div>
                <div class="card-content">
                    <span class="label">Graphics</span>
                    <span class="value">{{ sysInfo.gpu_info }}</span>
                </div>
            </div>
        </div>
    </div>
  </PageLayout>
</template>

<style scoped>
.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px;
    color: var(--text-secondary);
    gap: 15px;
}

.about-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.glass-panel {
    background: var(--card-bg);
    border-radius: 12px;
    border: 1px solid var(--card-border);
    box-shadow: 0 4px 6px -2px rgba(0, 0, 0, 0.05); /* Subtle shadow for depth */
}

.hero-section {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 20px;
    padding: 40px;
    text-align: left;
}

.os-logo {
    width: 110px;
    height: 110px;
    background: white; /* Assuming logo needs white bg as per screenshot avatar style or just container */
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.logo-img {
    height: 100%;
    aspect-ratio: 1/1;
    object-fit: cover;
    border-radius: 14px;
    display: block;
}

.os-title {
    width: 100%;
}
.os-title h2 {
    margin: 0 0 8px 0;
    font-size: 28px;
    font-weight: 700;
    color: var(--text-primary);
}

.subtitle {
    margin: 0;
    color: var(--text-secondary);
    font-size: 16px;
}

.disk-usage {
    margin-top: 10px;
    width: 100%;
}

.disk-info-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
    font-size: 13px;
}

.disk-label {
    color: var(--text-secondary);
    font-weight: 500;
}

.disk-stats {
    color: var(--text-primary);
    font-weight: 600;
}

.progress-track {
    height: 8px;
    background: rgba(128, 128, 128, 0.2);
    border-radius: 4px;
    overflow: hidden;
}

.progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #0099ff, #00cbff);
    border-radius: 4px;
    transition: width 0.6s cubic-bezier(0.22, 1, 0.36, 1);
}

.info-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr); /* 2 columns as per screenshot */
    gap: 20px;
}

@media (min-width: 1000px) {
    .info-grid {
        grid-template-columns: repeat(2, 1fr); /* Keep 2 columns for this specific look, or auto-fit if preferred, but screenshot shows 2x2 */
    }
}

.info-card {
    padding: 30px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    text-align: center;
    transition: transform 0.2s, box-shadow 0.2s;
    height: 100%;
    box-sizing: border-box;
    min-height: 160px;
}

.info-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(0,0,0,0.08);
}

.card-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: var(--bg-secondary); /* Darker container */
    display: flex;
    align-items: center;
    justify-content: center;
    color: #0099ff; /* Specific blue from screenshot */
    margin-bottom: 4px;
}

.card-icon i {
    font-size: 1.2rem;
}

.card-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    width: 100%;
}

.label {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--text-secondary);
    font-weight: 700;
    opacity: 0.8;
}

.value {
    font-size: 15px;
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1.4;
    max-width: 90%;
}
</style>
