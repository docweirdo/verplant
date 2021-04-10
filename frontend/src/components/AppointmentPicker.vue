<template>
  <div class="appointment-picker">
    <label>{{ currentTranslation.date }}</label>
    <Calendar v-model="day" class="cal" :inline="true" />
    <div class="time-wrapper">
      <label for="start">{{ currentTranslation.startTime }}:</label>
      <input id="start" type="time" v-model="startTime" class="time-picker" />
    </div>
    <div class="time-wrapper">
      <label for="end">{{ currentTranslation.endTime }}:</label>
      <input id="end" type="time" v-model="endTime" class="time-picker" />
    </div>
    <Button
      :label="currentTranslation.add"
      @click="returnData"
      :disabled="disableAddButton"
    />
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, Ref, ref, computed } from "vue";
import moment from "moment";

// Our stuff
import { currentTranslation } from "@/translations";
import { api, Appointment, AppointmentStatus } from "@/api";

// Foreign components
import Button from "primevue/button";
import Calendar from "primevue/calendar";

// Our components

export default defineComponent({
  name: "AppointmentPicker",
  components: {
    Button,
    Calendar,
  },
  props: {
    duration: Date,
  },
  setup(props) {
    const day = ref(null);
    const startTime = ref(null);
    const endTime = ref(null);
    const returnData = () => {
      console.log(day.value);
      console.log(startTime.value);
      console.log(endTime.value);
    };

    const disableAddButton = computed(() => {
      let missingInput: boolean = !(
        day.value &&
        startTime.value &&
        endTime.value
      );

      missingInput = missingInput && startTime.value < endTime.value; // Todo: Fix This
    });

    return {
      disableAddButton,
      day,
      currentTranslation,
      returnData,
      startTime,
      endTime,
    };
  },
});
</script>

<style scoped>
.appointment-picker {
  display: grid;
}

.time-wrapper {
  margin: 0.5em 0;
  display: grid;
  grid-template-columns: 1fr 2fr;
  justify-items: start;
}

.cal {
  min-height: 460px;
}

Button {
  margin-top: 1em;
  background-color: var(--accentColor);
}

Button:enabled:hover,
Button:enabled:focus {
  background-color: var(--accentColor);
}
</style>

<style>
.p-datepicker {
  border-color: rgba(0, 0, 0, 0.38) !important;
}
</style>