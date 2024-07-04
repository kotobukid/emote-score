<script setup lang="ts">
import {ref} from "vue"
import {invoke} from '@tauri-apps/api/tauri'
import FileInfo from "./FileInfo.vue"

const images = ref([]);
const current = ref('');

invoke('get_image_list').then(imageList => {
  images.value = imageList
});

const set_current = (i: string) => {
  current.value = i;
};
</script>

<template>
  <file-info
    v-for="i in images"
    :file="i"
    :current="current"
    @set-current="set_current"
  />
</template>

<style scoped lang="less">

</style>