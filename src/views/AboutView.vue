
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import logo from "../assets/logo.jpg";

interface SystemInfo {
    hostname: string;
    os_name: string;
    kernel_version: string;
    cpu_model: string;
    memory_total: string;
    gpu_info: string;
}

const loading = ref(true);
const sysInfo = ref<SystemInfo>({
    hostname: '',
    os_name: '',
    kernel_version: '',
    cpu_model: '',
    memory_total: '',
    gpu_info: ''
});

onMounted(async () => {
    try {
        sysInfo.value = await invoke('get_system_info');
    } catch (e) {
        console.error("Failed to get system info:", e);
    } finally {
        loading.value = false;
    }
});
</script>

<template>
  <div class="about-view">
    <div class="header">
        <h1 class="page-title">About System</h1>
    </div>

    <div v-if="loading" class="loading-container">
        <i class="pi pi-spin pi-spinner" style="font-size: 2rem"></i>
        <p>Fetching system information...</p>
    </div>

    <div v-else class="content-container">
        <div class="hero-section">
            <div class="os-logo">
                <img :src="logo" style="width: 100px; height: 100px; border-radius: 10px;" :alt="sysInfo.os_name">
            </div>
            <div class="os-title">
                <h2>{{ sysInfo.hostname }}</h2>
                <p class="subtitle">{{ sysInfo.os_name || 'Linux System' }}</p>
            </div>
        </div>

        <div class="info-grid">
            <div class="info-card">
                <div class="card-icon"><i class="pi pi-cog"></i></div>
                <div class="card-content">
                    <span class="label">Kernel</span>
                    <span class="value">{{ sysInfo.kernel_version }}</span>
                </div>
            </div>

            <div class="info-card">
                <div class="card-icon"><i class="pi pi-server"></i></div>
                <div class="card-content">
                    <span class="label">Processor</span>
                    <span class="value">{{ sysInfo.cpu_model }}</span>
                </div>
            </div>

            <div class="info-card">
                <div class="card-icon"><i class="pi pi-database"></i></div>
                <div class="card-content">
                    <span class="label">Memory</span>
                    <span class="value">{{ sysInfo.memory_total }}</span>
                </div>
            </div>
            
            <div class="info-card" v-if="sysInfo.gpu_info && sysInfo.gpu_info !== 'Unknown'">
                <div class="card-icon"><i class="pi pi-image"></i></div>
                <div class="card-content">
                    <span class="label">Graphics</span>
                    <span class="value">{{ sysInfo.gpu_info }}</span>
                </div>
            </div>
        </div>
    </div>
  </div>
</template>

<style scoped>
.about-view {
    padding: 0 40px 40px 40px;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
}

.header {
    padding: 20px 0 30px 0;
}

.page-title {
    font-size: 24px;
    font-weight: 700;
    margin: 0;
}

.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px;
    color: var(--text-secondary);
    gap: 15px;
}

.hero-section {
    display: flex;
    align-items: center;
    gap: 30px;
    margin-bottom: 40px;
    background: var(--card-bg, #fff);
    padding: 30px;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.03);
}

.os-logo {
    width: 100px;
    height: 100px;
    background: var(--bg-secondary, #f5f5f7);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.os-title h2 {
    margin: 0 0 5px 0;
    font-size: 28px;
    font-weight: 700;
}

.subtitle {
    margin: 0;
    color: var(--text-secondary);
    font-size: 16px;
}

.info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
}

.info-card {
    background: var(--card-bg, #fff);
    padding: 20px;
    border-radius: 12px;
    display: flex;
    align-items: center; /* Center vertically */
    gap: 15px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.02);
    border: 1px solid rgba(0,0,0,0.05);
    transition: transform 0.2s, box-shadow 0.2s;
}

.info-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(0,0,0,0.05);
}

.card-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: var(--bg-secondary, #f0f2f5);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent-color, #007aff);
}

.card-content {
    display: flex;
    flex-direction: column;
    overflow: hidden; /* Prevent text overflow */
}

.label {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary, #888);
    font-weight: 600;
    margin-bottom: 4px;
}

.value {
    font-size: 15px;
    font-weight: 500;
    word-break: break-word;
    line-height: 1.4;
}

@media (prefers-color-scheme: dark) {
    .hero-section, .info-card {
        background: rgba(255,255,255,0.05);
        border-color: rgba(255,255,255,0.05);
    }
    .os-logo {
        background: rgba(255,255,255,0.1);
    }
    .card-icon {
        background: rgba(255,255,255,0.1);
        color: #5ac8fa;
    }
}
</style>
