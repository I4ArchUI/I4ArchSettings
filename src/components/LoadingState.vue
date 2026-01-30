<script setup lang="ts">
defineProps<{
    loadingText?: string;
}>();

import loadingGif from '@/assets/loading-cat.gif';
</script>

<template>
    <div class="loading-container">
        <div class="loading-spinner">
            <img :src="loadingGif" alt="Loading" />
        </div>
        <p class="loading-text">{{ loadingText ?? 'Loading ...' }}</p>
    </div>
</template>

<style scoped>
.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 16px;
}

.loading-spinner img {
    width: 6rem;
    height: 6rem;
}

.loading-text {
    font-size: 1.2rem;
    color: var(--text-secondary);
    margin: 0;
}
</style>
