<script setup lang="ts">
import { useStartupViewModel } from '../viewmodels/startup.viewmodel';

const {
    loading,
    startupCommands,
    saveCommands,
    addCommand,
    removeCommand
} = useStartupViewModel();
</script>

<template>
    <div class="startup-view">
        <div class="header">
            <h1 class="page-title">
                Startup Applications
                <div style="display: flex; flex-direction: row; align-items: center; gap: 10px;">
                    <button class="secondary-btn" @click="addCommand">
                        <i class="pi pi-plus"></i> Add New
                    </button>
                    <button class="primary-btn small-btn" @click="saveCommands" :disabled="loading">
                        <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                        <i v-else class="pi pi-check"></i>
                        {{ loading ? 'Saving...' : 'Save Changes' }}
                    </button>
                </div>
            </h1>
        </div>

        <div class="content-container">
            <div class="settings-card glass-panel">
                <div class="commands-list">
                    <transition-group name="list">
                        <div v-for="(_, index) in startupCommands" :key="index" class="command-row">
                            <div class="input-wrapper">
                                <div class="badge">exec-once</div>
                                <i class="pi pi-equals operator"></i>
                                <input 
                                    type="text" 
                                    class="command-input" 
                                    v-model="startupCommands[index]" 
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
            </div>
        </div>
    </div>
</template>

<style scoped>
.startup-view {
    padding: 0 40px 40px 40px;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
}

.header {
    padding: 20px 0 30px 0;
}

.page-title {
    font-size: 28px;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary);
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.glass-panel {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    backdrop-filter: blur(20px);
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 4px 24px -1px rgba(0, 0, 0, 0.1);
}

.card-header {
    padding: 24px;
    display: flex;
    align-items: center;
    gap: 16px;
    border-bottom: 1px solid var(--card-border);
    background: rgba(255, 255, 255, 0.02);
}

.info-icon-wrapper {
    width: 44px;
    height: 44px;
    background: linear-gradient(135deg, #ff9966, #ff5e62);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 0 4px 12px rgba(255, 94, 98, 0.3);
}

.info-icon-wrapper i {
    font-size: 20px;
}

.info-title {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
    color: var(--text-primary);
}

.info-subtitle {
    margin: 4px 0 0 0;
    font-size: 0.9rem;
    color: var(--text-secondary);
}

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
    border: 1px solid transparent; /* var(--card-border) */
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

.row-handle {
    color: var(--text-secondary);
    cursor: grab;
    opacity: 0.3;
    padding: 4px;
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

.icon-btn {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    opacity: 0.5;
}

.command-row:hover .icon-btn {
    opacity: 1;
}

.delete-btn:hover {
    background-color: rgba(255, 59, 48, 0.15);
    color: #ff3b30;
}

.primary-btn, .secondary-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px 20px;
    border-radius: 10px;
    font-weight: 600;
    cursor: pointer;
    border: none;
    font-size: 0.95rem;
    transition: all 0.2s;
}

.primary-btn {
    background: var(--accent-color);
    color: white;
    box-shadow: 0 4px 12px rgba(var(--accent-rgb), 0.25);
}

.primary-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(var(--accent-rgb), 0.35);
}

.primary-btn:active:not(:disabled) {
    transform: translateY(1px);
}

.primary-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
}

.secondary-btn {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary);
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.secondary-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: var(--text-secondary);
}

.small-btn {
    padding: 8px 16px;
    font-size: 0.9rem;
    height: 36px;
}

.text-btn {
    background: none;
    border: none;
    color: var(--accent-color);
    font-weight: 600;
    cursor: pointer;
    font-size: 0.95rem;
}
.text-btn:hover {
    text-decoration: underline;
}

.empty-state {
    text-align: center;
    padding: 40px;
    color: var(--text-secondary);
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: center;
}

/* Animations */
.list-enter-active,
.list-leave-active {
    transition: all 0.3s ease;
}
.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(-10px);
}
</style>
