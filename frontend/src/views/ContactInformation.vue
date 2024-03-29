<template>
  <div class="contact-infos">
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
      id="course-dropdown"
      v-if="groupedCourses.length > 0"
      v-model="selectedCourse"
      v-bind:options="groupedCourses"
      optionLabel="name"
      optionGroupLabel="groupType"
      optionGroupChildren="courses"
      v-bind:placeholder="currentTranslation.selectedCoursePlaceholder"
      v-bind:filter="true"
      :disabled="store.bookingUrl.value"
    />
    <ProgressSpinner v-else id="course-spinner" />
    <hr />
    <div class="names split-lwi">
      <div class="label-with-info">
        <label for="first-name">{{ currentTranslation.firstname }}*</label>
        <i id="user-icon" class="pi pi-user info-icon" />
      </div>
      <div class="label-with-info">
        <label for="last-name">{{ currentTranslation.lastname }}*</label>
        <i class="pi pi-id-card info-icon" />
      </div>
      <InputText
        id="first-name"
        type="text"
        v-model="store.contactInformations.firstname"
        :disabled="store.bookingUrl.value"
      />
      <InputText
        id="last-name"
        type="text"
        v-model="store.contactInformations.lastname"
        :disabled="store.bookingUrl.value"
      />
    </div>
    <div class="contact-1 split-lwi">
      <div class="label-with-info">
        <label for="mail">{{ currentTranslation.email }}*</label>
        <i class="pi pi-envelope info-icon" />
      </div>
      <div class="label-with-info">
        <label for="phone">{{ currentTranslation.phone }}</label>
        <i class="pi pi-phone info-icon" />
      </div>

      <InputText
        id="mail"
        type="email"
        v-model="store.contactInformations.email"
        :disabled="store.bookingUrl.value"
      />
      <InputText
        id="phone"
        type="tel"
        v-model="store.contactInformations.phone"
        :disabled="store.bookingUrl.value"
      />
    </div>
    <div class="school-infos split-lwi">
      <div class="label-with-info">
        <label for="organisation">{{ currentTranslation.organisation }}</label>
        <i class="pi pi-globe info-icon" />
      </div>
      <div class="label-with-info">
        <label for="group">{{ currentTranslation.class }}</label>
        <i class="pi pi-users info-icon" />
      </div>

      <InputText
        id="organisation"
        type="text"
        v-model="store.contactInformations.organisation"
        :disabled="store.bookingUrl.value"
      />
      <InputText
        id="group"
        type="text"
        v-model="store.contactInformations.class"
        :disabled="store.bookingUrl.value"
      />
    </div>
    <div class="group-infos split-lwi">
      <div class="label-with-info">
        <label for="organisation">{{ currentTranslation.groupSize }}</label>
        <i class="pi pi-sort-numeric-up info-icon" />
      </div>
      <InputText
        id="group-size"
        type="number"
        v-model="store.contactInformations.groupSize"
        :disabled="store.bookingUrl.value"
      />
    </div>
    <!-- footer of card -->
    <InfoDialog ref="infoDialog" />
    <p>* {{ currentTranslation.mandatoryField }}</p>
    <hr />
    <div class="p-field-checkbox">
      <Checkbox
        id="legalNotice"
        v-model="store.contactInformations.acceptedLegalNotice"
        :binary="true"
        :disabled="store.bookingUrl.value"
      />
      <label for="legalNotice" v-html="currentTranslation.legalNotice"></label>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { computed, defineComponent, ref, watch } from "vue";

// Our stuff
import { api, Course, AppointmentSuggestion, AppointmentStatus } from "@/api";
import store from "@/store";
import { currentTranslation } from "@/translations";

// Foreign components
import InputText from "primevue/inputtext";
import Dropdown from "primevue/dropdown";
import Checkbox from "primevue/checkbox";
import ProgressSpinner from "primevue/progressspinner";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "Appointments",
  components: {
    InfoDialog,
    InputText,
    Dropdown,
    Checkbox,
    ProgressSpinner,
  },

  async setup() {
    const allCourses = store.allCourses;
    const selectedCourse = computed<Course | undefined>({
      get: () => {
        const a = store.allCourses.value.find(course => course.id === store.contactInformations.selectedCourse)
        console.log(a)
        return a
      },
      set: (val?: Course) => {
        console.log(val);
        
        store.contactInformations.selectedCourse = val?.id
      }
    });
    const infoDialog = ref(null);

    const courseTypes: Map<string, Course[]> = new Map();
    const groupedCourses = ref<{ courses: Course[]; groupType: string }[]>([]);

    const onCourseUpdate = (allCoursesUpdate: Course[]) => {
      // Split courses after types
      for (const result of allCoursesUpdate) {
        const cType = result.course_type ?? currentTranslation.miscCourseType;
        const c = courseTypes.get(cType);
        if (c) c.push(result);
        else courseTypes.set(cType, [result]);
      }
      console.log('setting new groupedCourses')
      for (const [key, value] of courseTypes.entries()) {
        groupedCourses.value.push({
          groupType: key,
          courses: value,
        });
      }
    }

    onCourseUpdate(store.allCourses.value)
    watch(allCourses, onCourseUpdate);


    return {
      currentTranslation,
      store,
      groupedCourses,
      selectedCourse,
      infoDialog,
    };
  },
});
</script>

<style scoped>
.contact-infos {
  width: 100%;
  display: grid;
  min-height: auto;
  grid-template-rows: repeat(7, min-content) auto repeat(2, min-content);
}

#course-spinner {
  width: 50px;
  height: 50px;
}
#course-dropdown {
  height: 50px;
}

.split-lwi {
  display: grid;
  grid-template-columns: 1fr 1fr;
  column-gap: 0.5em;
}

.info-icon {
  cursor: unset;
}

#user-icon {
  font-size: 0.8rem;
  padding: 0.1rem;
  border-radius: 50%;
  border: 0.1rem solid black;
}

#group-size {
  grid-row: 2 / span 1;
}

hr {
  margin: 0.8em 0 0 0;
  border: none;
  border-bottom: solid 0.1em black;
}

p {
  margin: 0.5em 0 0 0;
  align-self: end;
}

.p-field-checkbox {
  margin: 0.9em 0;
}

.p-field-checkbox > label {
  margin-left: 0.5em;
}

/* === MOBILE === */
@media only screen and (max-width: 520px) {
  .split-lwi {
    grid-template-columns: 1fr;
  }
  .split-lwi > .label-with-info:nth-of-type(2) {
    grid-row-start: 3;
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
</style>
