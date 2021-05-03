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
      v-model="selectedCourse"
      v-bind:options="groupedCourses"
      optionLabel="name"
      optionGroupLabel="groupType"
      optionGroupChildren="courses"
      v-bind:placeholder="currentTranslation.selectedCoursePlaceholder"
      v-bind:filter="true"
    />
    <hr />
    <div class="names">
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
      />
      <InputText
        id="last-name"
        type="text"
        v-model="store.contactInformations.lastname"
      />
    </div>
    <div class="contact-1">
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
      />
      <InputText
        id="phone"
        type="tel"
        v-model="store.contactInformations.phone"
      />
    </div>
    <div class="school-infos">
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
      />
      <InputText
        id="group"
        type="text"
        v-model="store.contactInformations.class"
      />
    </div>
    <div class="group-infos">
      <div class="label-with-info">
        <label for="organisation">{{ currentTranslation.groupSize }}</label>
        <i class="pi pi-globe info-icon" />
      </div>
      <InputText
        id="group-size"
        type="number"
        v-model="store.contactInformations.groupSize"
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
      />
      <label for="legalNotice" v-html="currentTranslation.legalNotice"></label>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { computed, defineComponent, ref } from "vue";

// Our stuff
import { api, Course, AppointmentSuggestion, AppointmentStatus } from "@/api";
import store from "@/store";
import { currentTranslation } from "@/translations";

// Foreign components
import InputText from "primevue/inputtext";
import Dropdown from "primevue/dropdown";
import Checkbox from "primevue/checkbox";

// Our Components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "Appointments",
  components: {
    InfoDialog,
    InputText,
    Dropdown,
    Checkbox,
  },

  async setup() {
    const apiResult = await api.getCourses();
    const selectedCourse = ref(null);
    const infoDialog = ref(null);

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
  /*min-height: min-content; to be figured out */
  grid-template-rows: repeat(7, min-content) auto repeat(2, min-content);
}

.names,
.contact-1,
.school-infos,
.group-infos {
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
