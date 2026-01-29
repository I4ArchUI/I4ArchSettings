<script setup lang="ts">
defineProps<{
    icon?: string;
    title?: string;
    subtitle?: string;
    iconBackground?: string;
}>();
</script>

<template>
    <div class="settings-card glass-panel">
        <div class="card-header" v-if="title || $slots.header">
            <template v-if="title">
                <div class="info-icon-wrapper" :style="{ background: iconBackground }">
                    <i :class="icon"></i>
                </div>
                <div class="info-content-text">
                    <h3>{{ title }}</h3>
                    <p v-if="subtitle">{{ subtitle }}</p>
                </div>
            </template>
            <slot name="header"></slot>
            <div style="margin-left: auto;" v-if="$slots.actions">
                <slot name="actions"></slot>
            </div>
        </div>
        
        <div class="card-content">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
.card-content {
    /* Padding might be handled by parent or specific to content */
    display: flex;
    flex-direction: column;
}
</style>
