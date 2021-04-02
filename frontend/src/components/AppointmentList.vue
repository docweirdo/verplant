<template>
    <Suspense>
        <div class="appointment-list">
            <div id="appointments" class="p-inputtextarea p-inputtext p-component">
                <div class="p-component p-card">test</div>
            </div>
            
            <InfoDialog ref="infoDialog"/>
        </div>
    </Suspense>
</template>

<script lang="ts">

// Foreign stuff
import { defineComponent, Ref, ref } from "vue";

// Our stuff
import { currentTranslation } from "@/translations";
import { api, Appointment} from "@/api"

// Foreign components

// Our components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "AppointmentList",
  components: {
      InfoDialog
  },
  props: {
      bookingId: Number,
  },
  async setup(props) {
    const infoDialog = ref(false);
    const appointments : Ref<Appointment[]> = ref([]);

    if (props.bookingId) appointments.value = await api.getAppointments(props.bookingId);


    return {
      currentTranslation,
      infoDialog
    }
  }
});
</script>

<style scoped>

</style>
