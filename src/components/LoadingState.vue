<script setup lang="ts">
// No props needed for this simple component
import loadingGif from '@/assets/loading-cat.gif';
</script>

<template>
    <div class="loading-container">
        <div class="loading-spinner">
            <img :src="loadingGif" alt="Loading" />
        </div>
        <p class="loading-text">Loading...</p>
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
    width: 3rem;
    height: 3rem;
}

.loading-text {
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin: 0;
}
</style>
