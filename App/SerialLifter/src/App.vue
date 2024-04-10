<template>
  <div class="container">
    <h1>Welcome to Serial Lifter!</h1>
    <button @click="open_port" style="max-width: 150px; margin: auto;">Open Port</button>
    <p style="font-size: 36pt;">Weight:</p>
    <p :style="{fontSize: '72pt', color: validReading ? 'green' : 'white' }">{{ scaleWeight }}</p>
    <p :style="{fontSize: '58pt'}">{{ highscore }}</p>
  </div>
</template>


<script setup>
import {ref} from 'vue';
import {listen} from '@tauri-apps/api/event';
import {invoke} from '@tauri-apps/api/core';

const open_port = async () => {
  try{
    await invoke('start_port_listen');
    console.log('Port opened');
  }catch(err){
    console.log('Port not opened', err);
  }
}
open_port();
const dataReading = ref("");
const scaleWeight = ref(0);
const validReading = ref(false);
const highscore = ref(0);

// listen for events coming from the backend
listen('new_reading', (event) => {
  dataReading.value = event.payload;
  parseReading(dataReading.value);
})

const parseReading = (reading) => {
  reading = reading.slice(1).replace(/\s/g,''); // the STX leading bit was causing issues
  const regex = /([0-9]+)/i;
  const match = regex.exec(reading);
  if (match) {
    scaleWeight.value = parseInt(match[1]);
    if (scaleWeight.value > highscore.value){
      highscore.value = scaleWeight.value;
    }
  } else {
    scaleWeight.value = 0;
  }
  // Checks if scale is in motion and determines the color of text
  if (reading[reading.length -1] === 'M'){
    validReading.value = false;
  } else {
    validReading.value = true;
  }

}

</script>
