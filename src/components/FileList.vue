<script setup lang="ts">
import { ref } from "vue"
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri';

const images = ref([]);
const current = ref('');

invoke('get_image_list', /* pass arguments */).then(imageList => {
  // use the imageList data
  images.value = imageList
});

const set_current = (i: string) => {
    current.value = i;
};
</script>

<template>
<ul>
    <li v-for="i in images"
        :class="current === i ? 'active': ''"
        @click="set_current(i)"
    >
        <img
         class="face" :key="i" :src="`${convertFileSrc(i)}`"
        >
    </li>
</ul>

</template>

<style scoped lang="less">
ul {
    list-style: none;
}

li {
    border: 3px solid grey;
    padding: 5px;
    margin: 5px;
}

li.active {
    border-color: blue;
    background-color: lightblue;
}

img.face {
    width: 256px;
    height: 256px;
}
</style>