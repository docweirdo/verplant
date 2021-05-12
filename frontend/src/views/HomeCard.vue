<template>
  <div class="home">
    <div class="home-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.loginCardTitle }}
      </h1>
      <Card class="center-card" :class="[`page-${site}`]">
        <template #content>
          <ContactInformation v-if="site == 0" />
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
        <div class="tooltip-wrapper" v-tooltip.left="missingFieldsTooltipText">
          <Button
            :label="buttonLabel"
            :disabled="!allMandatoryFilled"
            class="card-switch-button"
            @click="nextPageOrSend"
            :icon="loading ? 'pi pi-spin pi-spinner' : ''"
            :class="{ loading: loading }"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { computed, defineComponent, ref } from "vue";

// Our stuff
import { api, Course } from "@/api";
import store from "@/store";
import { currentTranslation } from "@/translations";
import router from "@/router";
import CustomerService from "@/CustomerService";

// Foreign Components
import Card from "primevue/card";
import Button from "primevue/button";
import Tooltip from "primevue/tooltip";

// Our Components
import Appointments from "@/views/Appointments.vue";
import ContactInformation from "@/views/ContactInformation.vue";
import { onBeforeRouteLeave } from "vue-router";
import { isEmail } from "@/utils";

export default defineComponent({
  name: "HomeCard",
  components: {
    Card,
    Button,
    Appointments,
    ContactInformation,
  },
  directives: {
    tooltip: Tooltip,
  },
  setup() {
    const site = ref(0);

    (window as any).site = site;

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
        await CustomerService.sendChanges();
        loading.value = false;
      }
    };

    // TODO: remove === SKIP FIRST PAGE ===
    nextPageOrSend();

    store.contactInformations.firstname = "Max";
    store.contactInformations.lastname = "Mustermann";
    store.contactInformations.email = "max.mustermann@gmail.com";
    store.contactInformations.organisation = "Tolle Schule Berlin";
    store.contactInformations.acceptedLegalNotice = true;

    // === END SKIP FIRST PAGE ===

    const popStateEventListener = (event: PopStateEvent) => {
      console.log("popState Event fired", event);
      if (site.value == 1) {
        site.value = 0;
      } else if (event.state === "fakePush" && site.value == 0) {
        site.value = 1;
      } else {
        window.removeEventListener("popstate", popStateEventListener);
      }
    };

    window.addEventListener("popstate", popStateEventListener);

    const allMandatoryFilled = computed(() => {
      return (
        store.contactInformations.firstname.length >= 1 &&
        store.contactInformations.lastname.length >= 1 &&
        isEmail(store.contactInformations.email) &&
        store.contactInformations.acceptedLegalNotice
      );
    });

    const missingFieldsTooltipText = computed(() => {
      if (allMandatoryFilled.value) return "";
      const missingFields: string[] = [];
      if (store.contactInformations.firstname.length < 1)
        missingFields.push(currentTranslation.firstname);
      if (store.contactInformations.lastname.length < 1)
        missingFields.push(currentTranslation.lastname);
      if (!isEmail(store.contactInformations.email))
        missingFields.push(currentTranslation.email);
      if (!store.contactInformations.acceptedLegalNotice)
        missingFields.push(currentTranslation.legalNoticeShort);

      if (missingFields.length == 1) {
        return `Feld ${missingFields[0]} fehlt.`; // TODO: allow interpolatable strings in currentTranslation
      }
      return `Folgende Felder sind Pflichtfelder: ${missingFields.join(", ")}`;
    });

    return {
      site,
      buttonLabel,
      nextPageOrSend,
      loading,
      currentTranslation,
      history,
      allMandatoryFilled,
      missingFieldsTooltipText,
    };
  },
});
</script>

<style scoped>
.home {
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
  margin: 0 auto;
  display: flex;
  min-height: min(calc(100vh - 200px), 700px);
  overflow: hidden;
}

.center-card.page-1 {
  height: min(calc(100vh - 200px), 700px);
}

#card-headline {
  font-size: 2em;
  font-weight: 500;
  color: white;
  margin-bottom: 5px;
}

@media only screen and (max-width: 520px) {
  .center-card {
    width: 100vw;
  }
}

@media only screen and (max-height: 700px) {
  .center-card {
    min-height: calc(100vh - 12em);
  }

  .center-card.page-1 {
    height: unset;
  }

  #card-headline {
    font-size: 1.2em;
  }
}

.card-switch-container {
  margin-top: 0.5em;
  margin-bottom: 2em;
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

.card-switch-button:disabled {
  background-color: #5f5f5f !important;
}
</style>

<style>
.center-card.p-card > .p-card-body,
.center-card.p-card > .p-card-body > .p-card-content {
  padding-top: 1px;
  padding-bottom: 1px;
  display: flex;
  width: 100%;
  /* position: absolute of transition is relative to nearest positioned element */
  position: relative;
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
