<template>
  <div class="appointment-picker">
    <label>{{ currentTranslation.date }}</label>
    <Calendar v-model="day" class="cal" :inline="true" :minDate="new Date()" />
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

// Our stuff
import { currentTranslation } from "@/translations";
import {
  api,
  Appointment,
  AppointmentStatus,
  AppointmentSuggestion,
} from "@/api";
import * as utils from "@/utils";

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
    duration: Date, // TODO: default duration?
  },
  emits: ["newAppointment"],
  setup(props, { emit }) {
    const day: Ref<Date | null> = ref(null);
    const startTime: Ref<string | null> = ref(null);
    const endTime: Ref<string | null> = ref(null);
    const returnData = () => {
      const myDay: Date | null = day.value;

      if (!myDay) {
        return;
      }

      const startHours: number = parseInt(
        startTime.value?.split(":")[0] ?? "0"
      );

      const startMinutes: number = parseInt(
        startTime.value?.split(":")[1] ?? "0"
      );

      const endHours: number = parseInt(endTime.value?.split(":")[0] ?? "0");

      const endMinutes: number = parseInt(endTime.value?.split(":")[1] ?? "0");

      const suggestion: AppointmentSuggestion = {
        from: utils.addHoursAndMinutes(myDay, startHours, startMinutes),
        to: utils.addHoursAndMinutes(myDay, endHours, endMinutes),
      };

      emit("newAppointment", suggestion);
    };

    const disableAddButton = computed(() => {
      let missingInput = !(day.value && startTime.value && endTime.value);

      let durationIsNegative = (startTime.value ?? "") >= (endTime.value ?? "");

      return durationIsNegative || missingInput;
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

.p-datepicker span {
  user-select: none;
}
</style>
