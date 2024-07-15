<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  invoke("greet", { name: name.value })
    .then((message) => {
      console.log(message)
      greetMsg.value = message
    })
    .catch((error) => console.error(error))
}

async function tauri_version() {
  invoke("tauri_version")
  .then((message) => {
    console.log(message);
  })
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>
  <br>
  <button style="width: 150px; margin-left: 35%;" v-cloak="tauri_version">获取系统版本</button>

  <p>{{ greetMsg }}</p>
</template>
