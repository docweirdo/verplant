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
    <hr>
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
        v-model="firstname"
        />
        <InputText
            id="last-name"
            type="text"
            v-model="lastname"
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
        v-model="email"
        />
        <InputText
        id="phone"
        type="tel"
        v-model="phone"
        />
    </div>
    <div class="school-infos">
        <div class="label-with-info">
            <label for="mail">{{ currentTranslation.organisation}}</label>
            <i class="pi pi-envelope info-icon" />
        </div>
        <div class="label-with-info">
            <label for="phone">{{ currentTranslation.class }}</label>
            <i class="pi pi-phone info-icon" />
        </div>
        
        
        <InputText
        id="mail"
        type="email"
        v-model="email"
        />
        <InputText
        id="phone"
        type="tel"
        v-model="phone"
        />
    </div>
    
    <div class="p-field-checkbox">
            <Checkbox id="legalNotice" v-model="acceptedLegalNotice" :binary="true" />
            <label for="legalNotice">{{currentTranslation.legalNotice}}.</label>
        </div>
    <InfoDialog ref="infoDialog" />
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { api, Course, AppointmentSuggestion, AppointmentStatus } from "@/api";
import { currentTranslation } from "@/translations";

// Foreign components
import InputText from 'primevue/inputtext';
import Dropdown from "primevue/dropdown";
import Checkbox from 'primevue/checkbox';


// Our Components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "Appointments",
  components: {
    InfoDialog,
    InputText,
    Dropdown,
    Checkbox
  },


  async setup() {
    const firstname = ref("");
    const lastname = ref("");
    const email = ref("");
    const phone = ref("");
    const organisation = ref("");
    const group = ref("");

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
        firstname,
        lastname,
        email,
        phone,
        organisation,
        group,
              groupedCourses,
      selectedCourse,
      infoDialog
    };
  },
});
</script>

<style scoped>

.contact-infos {
    width: 100%;
}

.names, .contact-1, .school-infos {
    display: grid;
    grid-template-columns: 1fr 1fr;
    column-gap: 0.5em;
}

.info-icon {
    cursor: unset;
}

#user-icon{
    font-size: 0.8rem;
    padding: 0.1rem;
    border-radius: 50%;
    border: 0.1rem solid black;
}

hr {
    margin: 0.8em 0 0 0;
    border: none;
    border-top: solid 0.1em black;

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