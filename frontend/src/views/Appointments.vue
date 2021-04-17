<template>
  <div class="appointments">
    <div id="infodiv-appoinmentList" class="label-with-info">
      <label>{{ currentTranslation.appointmentSuggestions }}</label>
      <i
        class="pi pi-question-circle info-icon"
        v-on:click="
          infoDialog.displayInfoDialog(
            currentTranslation.appointmentSuggestions,
            currentTranslation.information.appointmentList
          )
        "
      />
    </div>
    <AppointmentList
      class="appointment-list"
      :bookingURL="null"
      v-model:appointments="appointments"
    >
      <template #between>
        <Button
          icon="pi pi-plus"
          class="p-button-raised p-button-rounded add-appointment"
          @click="displayAppointmentPicker = true"
          style="background-color: var(--accentColor)"
        />
      </template>
    </AppointmentList>

    <InfoDialog ref="infoDialog" />
    <Dialog
      id="newAppointmentDialog"
      :header="currentTranslation.newAppointment"
      v-model:visible="displayAppointmentPicker"
      :modal="true"
      :dismissable-mask="true"
    >
      <AppointmentPicker @newAppointment="createAppointment" />
    </Dialog>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { api, Course, AppointmentSuggestion, AppointmentStatus } from "@/api";
import { currentTranslation } from "@/translations";

// Foreign Components
import Dialog from "primevue/dialog";
import Button from "primevue/button";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";
import AppointmentList from "@/components/AppointmentList.vue";
import AppointmentPicker from "@/components/AppointmentPicker.vue";

export default defineComponent({
  name: "Appointments",
  components: {
    InfoDialog,
    AppointmentList,
    Dialog,
    AppointmentPicker,
    Button,
  },
  async setup() {
    const infoDialog = ref(null);
    const displayAppointmentPicker = ref(false);



    const rawAppointments = await api.getAppointments("abcde"); // TODO: Booking ID Logic
    console.log(rawAppointments);
    const appointments = ref([...rawAppointments]);

    const createAppointment = (event: AppointmentSuggestion) => {
      console.log(event);
      displayAppointmentPicker.value = false;
      appointments.value.push({
        starttime: event.from,
        endtime: event.to,
        status: AppointmentStatus.Pending,
      });
    };

    return {
      appointments,
      currentTranslation,
      infoDialog,
      displayAppointmentPicker,
      createAppointment,
    };
  },
});
</script>

<style scoped>
.appointments {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
}
</style>

