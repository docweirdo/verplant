<template>
  <div class="home">
    <Card class="center-card">
      <template #title>
        {{ currentTranslation.bookingCardTitle }}
      </template>
      <template #content>
        <img alt="Vue logo" src="../assets/logo.png" />
        <Dropdown
          v-model="selectedCourse"
          v-bind:options="groupedCourses"
          optionLabel="name"
          optionGroupLabel="groupType"
          optionGroupChildren="courses"
          v-bind:placeholder="currentTranslation.selectedCoursePlaceholder"
          v-bind:filter="true"
        />
      </template>
    </Card>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import Card from "primevue/card";
import Dropdown from "primevue/dropdown";
import { currentTranslation } from "@/translations";
import * as api from "@/api";

export default defineComponent({
  name: "Home",
  components: {
    Card,
    Dropdown,
  },
  async setup() {
    const selectedCourse = ref(null);

    const apiResult = await api.getCourses();

    const courseTypes: Map<string, api.Course[]> = new Map();

    for (const result of apiResult) {
      const cType = result.course_type ?? currentTranslation.miscCourseType;
      const c = courseTypes.get(cType);
      if (c) c.push(result);
      else courseTypes.set(cType, [result]);
      
    }

    const groupedCourses: { courses: api.Course[]; groupType: string }[] = [];

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
  max-width: 400px;
  max-height: 600px;
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