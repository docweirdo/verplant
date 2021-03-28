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

interface Course {
  name: string;
  code: number;
}

export default defineComponent({
  name: "Home",
  components: {
    Card,
    Dropdown,
  },
  setup() {
    const selectedCourse = ref(null);

    const groupedCourses: { courses: Course[]; groupType: string }[] = [
      { groupType: "Digitales", 
        courses: [
          {name: "Photoshop", code: 1},
          {name: "Robotik", code: 2}
        ]
      },
      { groupType: "Zeichnen", 
        courses: [
          { name: "Moderne Kunst", code: 3 },
          { name: "Modellzeichnen", code: 4 },
        ]
      }
    ];

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