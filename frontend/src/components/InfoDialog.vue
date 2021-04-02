<template>
  <div class="info-dialog">
    <Dialog v-bind:header="`${currentTranslation.info}: ${header}`" v-model:visible="visible" :breakpoints="{'960px': '75vw', '640px': '100vw'}" :style="{width: '50vw'}" :modal="true" :dismissableMask="true">
    {{ msg }}
    </Dialog>
  </div>
</template>

<script lang="ts">

// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { currentTranslation } from "@/translations";

// Foreign components
import Dialog from 'primevue/dialog';

// Our components


export default defineComponent({
  name: "InfoDialog",
  components: {
    Dialog
  },
  setup() {
    const visible = ref(false);
    const msg = ref("");
    const header = ref("");

    return {
      currentTranslation,
      visible,
      msg,
      header,
      displayInfoDialog: (new_header: string, new_msg: string) => {
        
        msg.value = new_msg;
        header.value = new_header;
        visible.value = true;
      },
    }
  }
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
