<template>
  <Suspense>
    <router-view />
  </Suspense>
</template>


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
