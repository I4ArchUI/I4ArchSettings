<script setup lang="ts">
import { useEnvViewModel } from '../viewmodels/env.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';

const {
    loading,
    envVars,
    saveEnvVars,
    addEnvVar,
    removeEnvVar
} = useEnvViewModel();
</script>

<template>
    <PageLayout>
        <template #title>
            Environment Variables
        </template>
        <template #actions>
            <button class="secondary-btn" @click="addEnvVar">
                <i class="pi pi-plus"></i> Add New
            </button>
            <button class="primary-btn small-btn" @click="saveEnvVars" :disabled="loading">
                <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                <i v-else class="pi pi-check"></i>
                {{ loading ? 'Saving...' : 'Save Changes' }}
            </button>
        </template>

        <SettingsCard>
            <div class="env-list">
                <transition-group name="list">
                    <div v-for="(item, index) in envVars" :key="item.id" class="env-row">
                        <div class="input-wrapper">
                            <span class="prefix">env =</span>
                            <input 
                                type="text" 
                                class="env-input key-input" 
                                v-model="item.key" 
                                placeholder="VARIABLE"
                            >
                            <span class="separator">,</span>
                            <input 
                                type="text" 
                                class="env-input value-input" 
                                v-model="item.value" 
                                placeholder="Value"
                                @keydown.enter.prevent="saveEnvVars"
                            >
                        </div>
                        <button class="icon-btn delete-btn" @click="removeEnvVar(index)" title="Remove">
                            <i class="pi pi-times"></i>
                        </button>
                    </div>
                </transition-group>
            </div>

            <div v-if="envVars.length === 0" class="empty-state">
                 <p>No environment variables defined.</p>
                 <button class="text-btn" @click="addEnvVar">Add Variable</button>
            </div>
        </SettingsCard>
    </PageLayout>
</template>

<style scoped>
.env-list {
    display: flex;
    flex-direction: column;
    padding: 10px;
}

.env-row {
    display: flex;
    gap: 12px;
    align-items: center;
    padding: 10px 14px;
    background: var(--bg-secondary);
    border: 1px solid transparent;
    border-radius: 10px;
    margin-bottom: 8px;
    transition: all 0.2s;
}

.env-row:hover {
    background: var(--card-bg);
    border-color: var(--accent-color);
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
    transform: translateY(-1px);
}

.input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 10px;
}

.prefix {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.85rem;
    color: var(--text-secondary);
    user-select: none;
}

.separator {
    font-size: 1.2rem;
    color: var(--text-secondary);
    font-weight: bold;
    user-select: none;
}

.env-input {
    background: transparent;
    border: none;
    color: var(--text-primary);
    padding: 8px;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.95rem;
    outline: none;
    border-bottom: 1px solid transparent;
    transition: border-color 0.2s;
}

.env-input:focus {
    border-bottom-color: var(--accent-color);
}

.key-input {
    width: 200px;
    font-weight: 600;
    color: var(--accent-color);
}

.value-input {
    flex: 1;
}

.delete-btn {
    opacity: 0.6;
    transition: opacity 0.2s;
}

.delete-btn:hover {
    opacity: 1;
    color: #ff4d4d;
}
</style>
