<script setup lang="ts">
import {ref, provide, type Ref} from "vue"
import {invoke} from '@tauri-apps/api/tauri'
import FileInfo from "./FileInfo.vue"
import {appCacheDir} from "@tauri-apps/api/path";

const images: Ref<FileInfo[]> = ref([]);
const current = ref('');

type FileInfo = {
  file: string,
  disabled: boolean,
  tags: Map<EmoteKey, boolean>
}


const initialize_file_to_local = (file: string): FileInfo => {
  return {
    file,
    disabled: false,
    tags: new Map(emotionMap.entries())
  };
};

type EmoteKey = string;
type EmoteJp = string;
type Emotion = [EmoteKey, EmoteJp]

const available_tags: Emotion[] = [
  ['neutral', '標準'],
  ['smile', '笑顔'],
  ['happy', '幸福'],
  ['amazing', '称賛'],
  ['curious', '興味'],
  ['cheerful', '応援'],
  ['active', '意欲'],
  ['thinking', '考察'],
  ['flatter', '媚態'],
  ['sep', ''],
  ['sad', '悲哀'],
  ['angry', '怒り'],
  ['bored', '退屈'],
  ['annoying', '面倒'],
  ['sleepy', '眠気'],
  ['crying', '落涙'],
  ['shame', '恥辱'],
  ['shouting', '慟哭'],
  ['despise', '軽蔑'],
  ['disrespect', '失望'],
];

const emotionMap: Map<EmoteKey, boolean> = new Map();
available_tags.forEach(tag => {
  emotionMap.set(tag[0], false);
});

provide('available-tags', available_tags);

invoke('get_image_list').then(imageList => {
  images.value = imageList.map(initialize_file_to_local);
});

const set_current = (i: string) => {
  current.value = i;
};

const set_disabled = ({file, disabled}: { file: string, disabled: boolean }) => {
  for (let i = 0; i < images.value.length; i++) {
    const image = images.value[i];
    if (image.file === file) {
      image.disabled = disabled;
      break;
    }
  }

  images.value = [...images.value];
};

const set_tag = (info: {file: string, tag: EmoteKey, active: boolean}) => {
  for (let i = 0; i < images.value.length; i++) {
    const image = images.value[i];
    if (image.file === info.file) {
      image.tags.set(info.tag, info.active);
      break;
    }
  }

  images.value = [...images.value];
};

const clear_tags = (file: string) => {
  for (let i = 0; i < images.value.length; i++) {
    const image = images.value[i];
    if (image.file === file) {
      image.tags = new Map(emotionMap.entries());
      break;
    }
  }

  images.value = [...images.value];
}
</script>

<template>
  <file-info
    v-for="image in images"
    :file="image.file"
    :current="current"
    :tags="image.tags"
    :disabled="image.disabled"
    @set-current="set_current"
    @set-disabled="set_disabled"
    @set-tag="set_tag"
    @clear-tags="clear_tags"
  />
</template>

<style scoped lang="less">

</style>