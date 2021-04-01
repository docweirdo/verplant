<template>
  <div class="home">
    <Card class="center-card">
      <template #title>
        {{ currentTranslation.bookingCardTitle }}
      </template>
      <template #content>
        <h4>{{ currentTranslation.course }}</h4>
        <div class="field-with-info">
          <Dropdown
            v-model="selectedCourse"
            v-bind:options="groupedCourses"
            optionLabel="name"
            optionGroupLabel="groupType"
            optionGroupChildren="courses"
            v-bind:placeholder="currentTranslation.selectedCoursePlaceholder"
            v-bind:filter="true"
          />
          <i class="pi pi-question-circle info-icon" v-on:click="infoDialog.displayInfoDialog(currentTranslation.course, currentTranslation.information.courseSelection)"/>
        </div>
      </template>
    </Card>
    <InfoDialog ref="infoDialog"/>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import{  api, Course } from "@/api";
import { currentTranslation } from "@/translations";

// Foreign Components
import Card from "primevue/card";
import Dropdown from "primevue/dropdown";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";


export default defineComponent({
  name: "Home",
  components: {
    Card,
    Dropdown,
    InfoDialog,
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

    for (const [key, value] of courseTypes.entries()){
      groupedCourses.push({
        groupType: key,
        courses: value
      });
    }
      
    return {
      currentTranslation,
      groupedCourses,
      selectedCourse,
      infoDialog
    };
  },
});
</script>

<style scoped>
.home {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}

.center-card {
  width: calc(100% - 20px);
  max-width: 600px;
  max-height: 600px;
  margin: 0 auto;
}

h4 {
  margin: 0;
  margin-bottom: 2px;
}

.field-with-info {
  display: grid;
  grid-template-columns: auto 3em;
  align-items: center;
}

.field-with-info > .info-icon {
  font-size: 1.5rem;
  margin: 0 auto;
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