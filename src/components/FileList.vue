<script setup lang="ts">
import { ref } from "vue"
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri';

const images = ref([]);

invoke('get_image_list', /* pass arguments */).then(imageList => {
  // use the imageList data
  images.value = imageList
});
</script>

<template>
<ul>
    <li v-for="i in images">
        <img class="face" :key="i" :src="`${convertFileSrc(i)}`">
    </li>
</ul>

</template>

<style scoped lang="less">
ul {
    list-style: none;
}

img.face {
    width: 128px;
    height: 128px;
}
</style>