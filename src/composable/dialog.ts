import { ref } from 'vue';

export const dialogState = () => {
  const isOpen = ref(false);

  function closeDialog() {
    isOpen.value = false;
  }

  return [isOpen, closeDialog];
};
