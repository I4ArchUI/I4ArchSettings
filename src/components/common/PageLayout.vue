<template>
    <div class="settings-view">
        <header class="page-header">
            <h1 class="page-title">
                <slot name="title"></slot>
                <div class="actions" v-if="$slots.actions">
                    <slot name="actions"></slot>
                </div>
            </h1>
        </header>

        <div class="content-container">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
.content-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.actions {
    display: flex;
    gap: 10px;
}
</style>
