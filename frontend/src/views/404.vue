<template>
  <div class="not-found">
    <div class="not-found-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.pageNotFoundTitle }}
      </h1>
      <Card>
        <template #content>
          <div class="wrapper">
            <p>{{ currentTranslation.pageNotFoundText }}</p>
            <i class="pi pi-compass"></i>
          </div>
        </template>
      </Card>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent } from "vue";

// Our stuff
import { currentTranslation } from "@/translations";

// Foreign Components
import Card from "primevue/card";

// Our Components

export default defineComponent({
  name: "Login",
  components: {
    Card
  },
  setup() {
    return {
      currentTranslation
    }
  }
});
</script>

<style scoped>
.not-found {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}

.not-found-content {
  max-width: 500px;
}

.not-found .wrapper {
  display: grid;
  grid-template-rows: auto 5rem;
  align-items: center;
  justify-items: center;
  text-align: justify;
}

.not-found .wrapper p {
  margin: 0;
}

@keyframes rotate {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(360deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.not-found .wrapper i {
  font-size: 2.5em;
  animation: rotate 2s ease-in-out;
  animation-iteration-count: infinite;
}

#card-headline {
  font-size: 2em;
  font-weight: 500;
  color: white;
  margin-bottom: 5px;
}

@media only screen and (max-height: 900px) {
  #card-headline {
    font-size: 1.2em;
  }
}

.center-card {
  padding-top: 0px;
  margin: 0 auto;
}


</style>

<style>
.not-found .p-card .p-card-body,
.not-found .p-card .p-card-content {
  padding: 5px;
  margin: 0;
}
</style>
