<template>
  <div class="appointments">
    <div class="label-with-info">
      <label>{{ currentTranslation.course }}</label>
      <i
        class="pi pi-question-circle info-icon"
        v-on:click="
          infoDialog.displayInfoDialog(
            currentTranslation.course,
            currentTranslation.information.courseSelection
          )
        "
      />
    </div>
    <Dropdown
      v-model="selectedCourse"
      v-bind:options="groupedCourses"
      optionLabel="name"
      optionGroupLabel="groupType"
      optionGroupChildren="courses"
      v-bind:placeholder="currentTranslation.selectedCoursePlaceholder"
      v-bind:filter="true"
    />
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
import Card from "primevue/card";
import Dropdown from "primevue/dropdown";
import Dialog from "primevue/dialog";
import Button from "primevue/button";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";
import AppointmentList from "@/components/AppointmentList.vue";
import AppointmentPicker from "@/components/AppointmentPicker.vue";

export default defineComponent({
  name: "Appointments",
  components: {
    Dropdown,
    InfoDialog,
    AppointmentList,
    Dialog,
    AppointmentPicker,
    Button,
  },
  async setup() {
    const selectedCourse = ref(null);
    const infoDialog = ref(null);
    const displayAppointmentPicker = ref(false);

    const apiResult = await api.getCourses();

    const courseTypes: Map<string, Course[]> = new Map();

    // Split courses after types
    for (const result of apiResult) {
      const cType = result.course_type ?? currentTranslation.miscCourseType;
      const c = courseTypes.get(cType);
      if (c) c.push(result);
      else courseTypes.set(cType, [result]);
    }

    const groupedCourses: { courses: Course[]; groupType: string }[] = [];

    for (const [key, value] of courseTypes.entries()) {
      groupedCourses.push({
        groupType: key,
        courses: value,
      });
    }

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
      groupedCourses,
      selectedCourse,
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
  display: grid;
}
</style>


<style>
.p-dropdown-item {
  padding-left: 30px !important;
}
.p-dropdown-item-group {
  color: var(--text-color) !important;
  font-weight: bold !important;
}
</style>