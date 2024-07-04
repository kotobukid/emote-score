<script setup lang="ts">
import {convertFileSrc} from "@tauri-apps/api/tauri";

const {file} = defineProps<{
  file: string,
  current: string
}>();

const emits = defineEmits<{
  (e: 'set-current', value?: string): void
}>();

const set_current = (f: string) => {
  emits('set-current', f);
};
</script>

<template lang="pug">
    .emote(
        :class="current === file ? 'active': ''"
        @click="set_current(file)"
    )
        img.face(:src="`${convertFileSrc(file)}`")
        span {{ file }}
</template>

<style scoped lang="less">
.emote {
    border: 3px solid grey;
    padding: 5px;
    margin: 5px;
}

.emote.active {
    border-color: blue;
    background-color: lightblue;
}

img.face {
    width: 256px;
    height: 256px;
}
</style>