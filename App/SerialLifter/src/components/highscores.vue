<!-- This file is unused (feature for later) -->

<template>
    <table>
      <thead>
        <tr>
          <th>Username</th>
          <th>Highscore</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>{{ username }}</td>
          <td>{{ number }}</td>
        </tr>
      </tbody>
    </table>
  </template>
  
  <script>
  import { useUserStore } from '@/stores/user';
  export default {
    setup() {
      const userStore = useUserStore();
      return {
        username: userStore.username,
        number: userStore.number,
      };
    },
  };
  </script>
  
  