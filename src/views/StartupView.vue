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
                <button class="primary-btn small-btn" @click="saveCommands" :disabled="loading" style="margin-left: auto;">
                    <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                    <i v-else class="pi pi-save"></i>
                    {{ loading ? 'Saving...' : 'Save' }}
                </button>
            </h1>
        </div>

        <div class="content-container">
            <div class="settings-card">

                <div class="commands-list">
                    <div v-for="(cmd, index) in startupCommands" :key="index" class="command-row">
                        <div class="input-wrapper">
                            <span class="prefix">exec-once = </span>
                            <input 
                                type="text" 
                                class="command-input" 
                                v-model="startupCommands[index]" 
                                placeholder="command argument..."
                                @keydown.enter.prevent="saveCommands"
                            >
                        </div>
                        <button class="icon-btn delete-btn" @click="removeCommand(index)" title="Remove">
                            <i class="pi pi-trash"></i>
                        </button>
                    </div>
                </div>

                <div class="card-footer">
                    <button class="secondary-btn" @click="addCommand">
                        <i class="pi pi-plus"></i> Add Command
                    </button>
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
    font-size: 24px;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 10px;
}

.content-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.settings-card {
    background-color: var(--card-bg);
    border-radius: 12px;
    padding: 20px;
    border: 1px solid var(--card-border);
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.card-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 10px;
}

.info-icon-wrapper {
    width: 40px;
    height: 40px;
    background-color: var(--bg-secondary);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent-color);
}

.info-icon-wrapper i {
    font-size: 20px;
}

.info-title {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
}

.info-subtitle {
    margin: 4px 0 0 0;
    font-size: 0.85rem;
    color: var(--text-secondary);
}

.commands-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.command-row {
    display: flex;
    gap: 10px;
    align-items: center;
}

.input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    background-color: var(--bg-secondary);
    border: 1px solid var(--input-border, rgba(255,255,255,0.1));
    border-radius: 8px;
    padding: 0 12px;
    transition: border-color 0.2s;
}

.input-wrapper:focus-within {
    border-color: var(--accent-color);
}

.prefix {
    color: var(--text-secondary);
    font-family: monospace;
    font-size: 0.9rem;
    user-select: none;
    margin-right: 4px;
}

.command-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    padding: 10px 0;
    font-family: monospace;
    font-size: 0.95rem;
    outline: none;
}

.icon-btn {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    border: 1px solid transparent;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
}

.delete-btn:hover {
    background-color: rgba(255, 59, 48, 0.1);
    color: #ff3b30;
}

.primary-btn, .secondary-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    font-size: 0.9rem;
    transition: all 0.2s;
}

.primary-btn {
    background-color: var(--accent-color);
    color: white;
}

.primary-btn:hover {
    filter: brightness(1.1);
}

.primary-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.secondary-btn {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--card-border);
}

.secondary-btn:hover {
    background-color: var(--card-bg);
    border-color: var(--accent-color);
}

.card-footer {
    display: flex;
    justify-content: flex-start;
    padding-top: 10px;
}
</style>
