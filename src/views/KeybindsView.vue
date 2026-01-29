<script setup lang="ts">
import { useKeybindsViewModel } from '../viewmodels/keybinds.viewmodel';

const {
    loading,
    keybinds,
    modifierOptions,
    bindTypes,
    loadKeybinds,
    saveKeybinds,
    addKeybind,
    removeKeybind
} = useKeybindsViewModel();
</script>

<template>

    <div class="keybinds-view">
        <div class="header">
            <h1 class="page-title">
                Keybinds
                <div class="actions">
                    <button class="secondary-btn small-btn" @click="loadKeybinds" :disabled="loading">
                        <i class="pi pi-refresh" :class="{ 'pi-spin': loading }"></i>
                    </button>
                    <button class="secondary-btn" @click="addKeybind" style="margin-left: auto;">
                        <i class="pi pi-plus"></i> Add New
                    </button>
                    <button class="primary-btn small-btn" @click="saveKeybinds" :disabled="loading">
                        <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                        <i v-else class="pi pi-check"></i>
                        {{ loading ? 'Saving...' : 'Save Changes' }}
                    </button>
                </div>
            </h1>
        </div>

        <div class="content-container">
            <div class="settings-card glass-panel">
                <div class="table-container">
                    <!-- Table Header -->
                     <div class="keybind-header-row">
                         <div class="col-type">Type</div>
                         <div class="col-mod">Modifier</div>
                         <div class="col-key">Key</div>
                         <div class="col-disp">Dispatcher</div>
                         <div class="col-args">Args</div>
                         <div class="col-action"></div>
                     </div>

                    <div class="keybinds-list">
                        <transition-group name="list">
                            <div v-for="(kb, index) in keybinds" :key="kb.id" class="keybind-row">
                                <!-- Type -->
                                <div class="col-type">
                                    <div class="select-wrapper">
                                        <select v-model="kb.bind_type" class="styled-select" title="Bind Type">
                                            <option v-for="opt in bindTypes" :key="opt.value" :value="opt.value">
                                                {{ opt.label }}
                                            </option>
                                        </select>
                                        <i class="pi pi-chevron-down select-icon"></i>
                                    </div>
                                </div>

                                <!-- Modifiers -->
                                <div class="col-mod">
                                    <div class="select-wrapper">
                                        <select v-model="kb.modifiers" class="styled-select" title="Modifiers">
                                            <option v-for="opt in modifierOptions" :key="opt.value" :value="opt.value">
                                                {{ opt.label }}
                                            </option>
                                        </select>
                                        <i class="pi pi-chevron-down select-icon"></i>
                                    </div>
                                </div>

                                <!-- Key -->
                                <div class="col-key">
                                    <input type="text" v-model="kb.key" class="styled-input code-font center-text" placeholder="Key">
                                </div>

                                <!-- Dispatcher -->
                                <div class="col-disp">
                                    <input type="text" v-model="kb.dispatcher" class="styled-input code-font" placeholder="Dispatcher">
                                </div>

                                <!-- Args -->
                                <div class="col-args">
                                    <input type="text" v-model="kb.args" class="styled-input code-font" placeholder="Arguments">
                                </div>

                                <!-- Action -->
                                <div class="col-action">
                                    <button class="icon-btn delete-btn" @click="removeKeybind(index)" title="Remove">
                                        <i class="pi pi-times"></i>
                                    </button>
                                </div>
                            </div>
                        </transition-group>
                    </div>
                    
                    <div v-if="keybinds.length === 0" class="empty-state">
                        <i class="pi pi-inbox empty-icon"></i>
                        <p>No keybinds found. Start by adding one.</p>
                        <button class="text-btn" @click="addKeybind">Add Keybind</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.keybinds-view {
    padding: 0 40px 40px 40px;
    max-width: 1100px;
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

.actions {
    display: flex;
    gap: 10px;
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
    background: linear-gradient(135deg, var(--accent-color), #8e2de2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 0 4px 12px rgba(var(--accent-rgb), 0.3);
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

.table-container {
    padding: 0;
}

.keybind-header-row {
    display: flex;
    gap: 12px;
    padding: 12px 24px;
    background: rgba(0, 0, 0, 0.1);
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
    font-weight: 600;
    border-bottom: 1px solid var(--card-border);
}

.keybinds-list {
    display: flex;
    flex-direction: column;
}

.keybind-row {
    display: flex;
    gap: 12px;
    align-items: center;
    padding: 12px 24px;
    border-bottom: 1px solid var(--card-border);
    transition: background-color 0.2s;
}

.keybind-row:hover {
    background-color: rgba(255, 255, 255, 0.03);
}

.keybind-row:last-child {
    border-bottom: none;
}

/* Columns */
.col-type { flex: 0 0 140px; }
.col-mod { flex: 0 0 120px; }
.col-key { flex: 0 0 80px; }
.col-disp { flex: 1; }
.col-args { flex: 1.5; }
.col-action { flex: 0 0 40px; display: flex; justify-content: flex-end; }

/* Inputs & Selects */
.select-wrapper {
    position: relative;
    width: 100%;
}

.styled-select, .styled-input {
    width: 100%;
    background: var(--bg-secondary);
    border: 1px solid transparent;
    color: var(--text-primary);
    padding: 8px 12px;
    border-radius: 8px;
    font-size: 0.9rem;
    height: 40px;
    transition: all 0.2s;
}

.styled-select {
    appearance: none;
    cursor: pointer;
    padding-right: 30px;
}

.select-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    font-size: 0.7rem;
    color: var(--text-secondary);
}

.styled-input:focus, .styled-select:focus {
    outline: none;
    background: var(--card-bg);
    border-color: var(--accent-color);
    box-shadow: 0 0 0 2px rgba(var(--accent-rgb), 0.1);
}

.code-font {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.85rem;
}

.center-text {
    text-align: center;
}

/* Buttons */
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
    opacity: 0.6;
}

.keybind-row:hover .icon-btn {
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
    transform: none;
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
    padding: 0;
    font-size: 0.95rem;
}
.text-btn:hover {
    text-decoration: underline;
}

.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: var(--text-secondary);
    gap: 12px;
}

.empty-icon {
    font-size: 3rem;
    margin-bottom: 8px;
    opacity: 0.3;
}

/* Animations */
.list-enter-active,
.list-leave-active {
    transition: all 0.3s ease;
}
.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(-20px);
}
</style>
