<script setup lang="ts">
import { useKeybindsViewModel } from '../viewmodels/keybinds.viewmodel';
import PageLayout from '../components/common/PageLayout.vue';
import SettingsCard from '../components/common/SettingsCard.vue';

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
    <PageLayout>
        <template #title>
            Keybinds
        </template>
        <template #actions>
            <button class="secondary-btn small-btn" @click="loadKeybinds" :disabled="loading" title="Refresh">
                <i class="pi pi-refresh" :class="{ 'pi-spin': loading }"></i>
            </button>
            <button class="secondary-btn small-btn" @click="addKeybind">
                <i class="pi pi-plus"></i> Add New
            </button>
            <button class="primary-btn small-btn" @click="saveKeybinds" :disabled="loading">
                <i v-if="loading" class="pi pi-spin pi-spinner"></i>
                <i v-else class="pi pi-check"></i>
                {{ loading ? 'Saving...' : 'Save Changes' }}
            </button>
        </template>

        <SettingsCard>
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
        </SettingsCard>
    </PageLayout>
</template>

<style scoped>
/* Only view-specific styles remain */
.table-container {
    padding: 0;
}

.keybind-header-row {
    display: flex;
    gap: 12px;
    padding: 12px 24px;
    background: rgba(0, 0, 0, 0.03);
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
    background-color: var(--item-hover-bg);
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
</style>
