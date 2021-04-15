<template>
  <div class="home">
    <div class="home-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.loginCardTitle }}
      </h1>
      <Card class="center-card">
        <template #content>
          <Appointments />
        </template>
      </Card>
      <div class="card-switch-container">
        <Button
          label=""
          id="card-switch-button"
          @click="login"
          :icon="loading ? 'pi pi-spin pi-spinner' : ''"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { api, Course } from "@/api";
import { currentTranslation } from "@/translations";
import router from "@/router";

// Foreign Components
import Card from "primevue/card";
import Button from "primevue/button";

// Our Components
import Appointments from "@/views/Appointments.vue";

export default defineComponent({
  name: "HomeCard",
  components: {
    Card,
    Button,
    Appointments,
  },
  setup() {
    const site = ref(0);

    return {
      site,
      currentTranslation,
    };
  },
});
</script>

<style scoped>
.home {
  width: 100vw;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}
.center-card {
  padding-top: 0px;
  width: calc(100vw - 20px);
  max-width: 600px;
  max-height: min(calc(100vh - 200px), 700px);
  margin: 0 auto;
  display: flex;
}

#card-headline {
  font-size: 2em;
  font-weight: 500;
  color: white;
  margin-bottom: 5px;
}

@media only screen and (max-height: 700px) {
  #card-headline {
    font-size: 1.2em;
  }
}
</style>

<style>
.center-card.p-card > .p-card-body,
.center-card.p-card > .p-card-body > .p-card-content {
  padding-top: 1px;
  display: flex;
}

.label-with-info {
  align-items: center;
}

.info-icon {
  font-size: 1rem;
  margin-left: 5px;
  cursor: pointer;
}

.label-with-info {
  margin-bottom: 0.2em;
  margin-top: 1em;
  display: flex;
  align-items: center;
}

.info-icon {
  font-size: 1rem;
  margin-left: 5px;
  cursor: pointer;
}
</style>