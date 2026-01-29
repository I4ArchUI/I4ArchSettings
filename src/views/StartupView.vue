<script setup lang="ts">
import { useStartupViewModel } from '../viewmodels/startup.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';

const {
    loading,
    startupCommands,
    saveCommands,
    addCommand,
    removeCommand
} = useStartupViewModel();
</script>

<template>
    <PageLayout>
        <template #title>
            Startup Applications
        </template>
        <template #actions>
            <button class="secondary-btn" @click="addCommand">
                <i class="pi pi-plus"></i> Add New
            </button>
            <button class="primary-btn small-btn" @click="saveCommands" :disabled="loading">
                <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                <i v-else class="pi pi-check"></i>
                {{ loading ? 'Saving...' : 'Save Changes' }}
            </button>
        </template>

        <SettingsCard>
            <div class="commands-list">
                <transition-group name="list">
                    <div v-for="(item, index) in startupCommands" :key="item.id" class="command-row">
                        <div class="input-wrapper">
                            <div class="badge">exec-once</div>
                            <i class="pi pi-equals operator"></i>
                            <input 
                                type="text" 
                                class="command-input" 
                                v-model="item.command" 
                                placeholder="Enter command (e.g. waybar)"
                                @keydown.enter.prevent="saveCommands"
                            >
                        </div>
                        <button class="icon-btn delete-btn" @click="removeCommand(index)" title="Remove">
                            <i class="pi pi-times"></i>
                        </button>
                    </div>
                </transition-group>
            </div>

            <div v-if="startupCommands.length === 0" class="empty-state">
                 <p>No startup commands defined.</p>
                 <button class="text-btn" @click="addCommand">Add Command</button>
            </div>
        </SettingsCard>
    </PageLayout>
</template>

<style scoped>
/* View-specific styles */
.commands-list {
    display: flex;
    flex-direction: column;
    padding: 10px;
}

.command-row {
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

.command-row:hover {
    background: var(--card-bg);
    border-color: var(--accent-color);
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
    transform: translateY(-1px);
}

.input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 12px;
}

.badge {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.8rem;
    color: var(--accent-color);
    background: rgba(var(--accent-rgb), 0.1);
    padding: 4px 8px;
    border-radius: 6px;
    user-select: none;
    font-weight: 600;
}

.operator {
    font-size: 0.7rem;
    color: var(--text-secondary);
    opacity: 0.5;
}

.command-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    padding: 8px 0;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.95rem;
    outline: none;
    width: 100%;
}

.command-row:hover .icon-btn {
    opacity: 1;
}
</style>
