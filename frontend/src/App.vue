<template>
  <Suspense>
    <router-view />
  </Suspense>
</template>
<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  mounted() {
    setTimeout(() => {
      // remove loader page after home and all child components are mounted (+ 1s for asthetics)
      const loader = document.querySelector('#loader-container') as HTMLDivElement;
      loader.style.opacity = '0'
      setTimeout(() => {
        loader.remove();
        console.log('removed loader container');
      }, 500); // opacity transition is 0.5s long
    }, 500);
  }
})
</script>

<style>
body {
  margin: 0;
  --accentColor: #96146e;
  background-color: var(--surface-800);
  background-image: url("./assets/splash01.svg");
  background-repeat: no-repeat;
  background-position: center;
  background-size: 100%;
}

.info-dialog {
  display: none;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  /*text-align: center; */
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
