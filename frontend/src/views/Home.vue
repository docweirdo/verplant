<template>
  <div class="home">
    <div class="home-content">
      <h1 id="card-headline" class="p-component"> {{ currentTranslation.bookingCardTitle }}</h1>
      <Card class="center-card">
        <template #content>
          <div class="label-with-info">
            <label>{{
              currentTranslation.course
            }}</label>
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
            <label>{{
              currentTranslation.appointmentSuggestions
            }}</label>
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
          <AppointmentList class="appointment-list" :bookingId="null" />
        </template>
      </Card>
      <InfoDialog ref="infoDialog" />
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { api, Course } from "@/api";
import { currentTranslation } from "@/translations";

// Foreign Components
import Card from "primevue/card";
import Dropdown from "primevue/dropdown";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";
import AppointmentList from "@/components/AppointmentList.vue";

export default defineComponent({
  name: "Home",
  components: {
    Card,
    Dropdown,
    InfoDialog,
    AppointmentList,
  },
  async setup() {
    const selectedCourse = ref(null);
    const infoDialog = ref(null);

    const apiResult = await api.getCourses();

    const courseTypes: Map<string, Course[]> = new Map();

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

    return {
      currentTranslation,
      groupedCourses,
      selectedCourse,
      infoDialog,
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
  background-color: var(--surface-800);
}

#card-headline {
  font-size: 2em;
  font-weight: 500;
  color: white;
  margin-bottom: 5px;
}

@media only screen and (max-height: 600px){
  #card-headline {
    font-size: 1.2em;
  }
}


.center-card {
  padding-top: 0px;
  width: calc(100vw - 20px);
  max-width: 600px;
  margin: 0 auto;
}

label {
  font-weight: bold;
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
}

::v-deep(#appointments) {
  height: calc(100vh - 400px);
  max-height: 400px;
  overflow-y: scroll;
}

@media only screen and (max-height: 600px) {
  ::v-deep(#appointments) {
    height: unset;
    max-height: unset;
  }
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

.center-card.p-card > .p-card-body, .center-card.p-card > .p-card-body > .p-card-content {
  padding-top: 1px;
}

</style>