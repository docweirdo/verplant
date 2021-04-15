<template>
  <div class="home">
    <div class="home-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.loginCardTitle }}
      </h1>
      <Card class="center-card">
        <template #content>
          <div class="placeholder" v-if="site == 0">SEITE 1</div>
          <Appointments v-else-if="site == 1" />
        </template>
      </Card>
      <div class="card-switch-container">
        <Button
          :style="{ visibility: site == 1 ? 'visible' : 'hidden' }"
          :label="currentTranslation.back"
          class="card-switch-button"
          @click="history.back()"
        />
        <Button
          :label="buttonLabel"
          class="card-switch-button"
          @click="nextPageOrSend"
          :icon="loading ? 'pi pi-spin pi-spinner' : ''"
          :class="{ loading: loading }"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { computed, defineComponent, ref } from "vue";

// Our stuff
import { api, Course } from "@/api";
import { currentTranslation } from "@/translations";
import router from "@/router";

// Foreign Components
import Card from "primevue/card";
import Button from "primevue/button";

// Our Components
import Appointments from "@/views/Appointments.vue";
import { onBeforeRouteLeave } from "vue-router";

export default defineComponent({
  name: "HomeCard",
  components: {
    Card,
    Button,
    Appointments,
  },
  setup() {
    const site = ref(0);
    const loading = ref(false);

    const buttonLabel = computed(() => {
      if (loading.value) return "";
      switch (site.value) {
        case 0:
          return currentTranslation.next;
        case 1:
          return currentTranslation.send;
        default:
          console.warn("HomeCard encountered error in switch statement");
          return null;
      }
    });

    const nextPageOrSend = async () => {
      if (site.value == 0) {
        site.value += 1;
        history.pushState("fakePush", "");
      } else if (site.value == 1) {
        // Todo: API Call
        loading.value = true;
        await new Promise((r) => setTimeout(r, 1500));
        loading.value = false;
      }
    };

    const popStateEventListener = (event: PopStateEvent) => {
      console.log("popState Event fired", event);
      if (site.value == 1) {
        site.value = 0;
      } else {
        window.removeEventListener("popstate", popStateEventListener);
      }
    };

    window.addEventListener("popstate", popStateEventListener);

    return {
      site,
      buttonLabel,
      nextPageOrSend,
      loading,
      currentTranslation,
      history,
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
  height: min(calc(100vh - 200px), 700px);
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

.card-switch-container {
  margin-top: 0.5em;
  display: flex;
  justify-content: space-between;
}

.card-switch-button,
.card-switch-button:enabled:hover,
.card-switch-button:enabled:focus {
  background-color: white;
  color: var(--accentColor);
  width: 150px;
  transition: width 0.3s ease-in-out;
}

.card-switch-button.loading,
.card-switch-button.loading:enabled:hover,
.card-switch-button.loading:enabled:focus {
  width: 50px;
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