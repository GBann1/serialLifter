// This file is unused (feature for later)

import { defineStore } from 'pinia';

export const useUserStore = defineStore('user', {
  state: () => ({
    username: '',
    number: 0,
  }),
  actions: {
    async loadFromIndexedDB() {
      // Code to open IndexedDB and retrieve username and number
      const db = await openIndexedDB();
      const transaction = db.transaction('userData', 'readonly');
      const store = transaction.objectStore('userData');
      const request = store.get('user');
      request.onsuccess = (event) => {
        const data = event.target.result;
        if (data) {
          this.username = data.username;
          this.number = data.number;
        }
      };
    },
    async saveToIndexedDB() {
      const db = await openIndexedDB();
      const transaction = db.transaction('userData', 'readwrite');
      const store = transaction.objectStore('userData');
      const request = store.put({ username: this.username, number: this.number });
      request.onsuccess = () => {
        console.log('Data saved to IndexedDB');
      };
      request.onerror = (error) => {
        console.error('Error saving data:', error);
      };
    },
  },
});

function openIndexedDB() {
  // Code to open IndexedDB connection and create object store if it doesn't exist
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('user-data', 1);
    request.onupgradeneeded = (event) => {
      const db = event.target.result;
      const objectStore = db.createObjectStore('userData', { keyPath: 'id' });
      objectStore.put({ id: 'user', username: '', number: 0 });
    };
    request.onsuccess = (event) => {
      resolve(event.target.result);
    };
    request.onerror = (error) => {
      reject(error);
    };
  });
}
