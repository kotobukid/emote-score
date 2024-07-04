<script setup lang="ts">
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {computed, inject} from "vue";

const props = defineProps<{
  file: string,
  current: string,
  disabled: boolean,
  tags: Map<string, boolean>
}>();

const emits = defineEmits<{
  (e: 'set-current', value?: string): void
  (e: 'clear-tags', value?: string): void
  (e: 'set-disabled', value?: {file: string, disabled: boolean}): void
  (e: 'set-tag', value?: {file: string, tag: string, active: boolean}): void
}>();

const available_tags = inject('available-tags');

const set_current = () => {
  emits('set-current', props.file);
};

const class_status = computed(() => {
  if (props.disabled) {
    return 'disabled';
  } else {
    return props.current === props.file ? 'active' : '';
  }
});

const toggle_disabled = () => {
  emits('set-disabled', {
    disabled: !props.disabled,
    file: props.file
  });
};

// const tag_active = computed((tag: string): boolean => {
//   return props.tags.has(tag) ? props.tags.get(tag) : false;
// });

const tag_active = (tag: string): boolean => {
  return props.tags.has(tag) ? props.tags.get(tag) : false;
};

const toggle_tag_active = (tag: string) => {
  emits('set-tag', {
    file: props.file,
    tag,
    active: !props.tags.get(tag)
  });
};

const clear_tags = () => {
  emits('clear-tags', props.file);
};
</script>

<template lang="pug">
  .emote(
    :class="class_status"
    @click="set_current"
  )
    .avatar
      img.face(:src="`${convertFileSrc(props.file)}`")
    .contents
      // span {{ props.file }}
      // br
      button.toggle_disabled(@click.stop="toggle_disabled" :class="props.disabled ? 'current_disabled' : 'current_active'")
        span(v-if="!props.disabled") 不使用にする
        span(v-else="props.disabled") 使用可にする
      hr
      .tags
        span.tag(
          v-for="tag in available_tags"
          )
            br(v-if="tag[0] === 'sep'")
            button.tag(
              v-else
              :class="tag_active(tag[0]) ? 'tag_active': ''"
              @click.stop="toggle_tag_active(tag[0])"
              v-text="tag[1]"
            )
      button.clear(@click.stop="clear_tags") クリア
</template>

<style scoped lang="less">
.emote {
  border: 3px solid grey;
  padding: 5px;
  margin: 5px;
  display: flex;

  &.disabled {
    background-color: grey;
    border-color: black;
  }

  &.active {
    border-color: blue;
    background-color: lightblue;
  }
}

.avatar {
  flex: 0 0 @image_width;
}

.contents {
  margin-left: 10px;
  flex: 1;
}

@image_width: 200px;

img.face {
  width: @image_width;
  height: @image_width;
}

div.tags {
  margin-bottom: 5px;
}

button.toggle_disabled {
  &.current_disabled {
    background-color: white;
    color: black;
  }
  &.current_active {
    background-color: grey;
    color: white;
  }
}

button.tag {
  margin: 2px;
  padding: 4px;
  border-radius: 5px;
  cursor: pointer;

  &:active {
    position: relative;
    top: 1px;
  }

  &.tag_active {
    background-color: #077607;
    color: #d5ffd5;
  }
}
</style>