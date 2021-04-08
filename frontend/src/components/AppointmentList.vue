<template>
    <Suspense>
        <div class="appointment-list">
            <div id="appointments" class="p-inputtextarea p-inputtext p-component">
                <div class="p-component p-card p-shadow-10" v-for="appointment in filteredAppointments" :key="appointment.id">
                  <div class="date">{{ toDateString(appointment.starttime) }}</div>
                  <div class="time">{{ toTimeString(appointment.starttime) }} - {{ toTimeString(appointment.endtime) }}</div>
                  <div class="status">{{ currentTranslation.appointmentState[appointment.status] ?? appointment.status }}</div>
                  <div class="controls" v-if="appointment.status != ''">
                    <span class="p-buttonset" v-if="appointment.status == 'SUGGESTED'">
                      <Button class="approve" icon="pi pi-check"/>
                      <Button class="reject" icon="pi pi-times"/>
                    </span>
                    <i class="approved pi pi-check-circle" v-else-if="appointment.status == 'APPROVED'" />
                    <Button class="pending" v-else-if="appointment.status == 'PENDING'" icon="pi pi-undo"/>
                    <Button class="rejected" disabled v-else-if="appointment.status == 'REJECTED'" icon="pi pi-times"/>
                  </div>
                </div>
                <div class="empty" v-if="appointments.length == 0">
                  <i class="pi pi-info-circle"></i>
                  {{ currentTranslation.noAppointmentsMessage }}
                </div>
            </div>
            
            <div id="filters">
               <Chip v-for="f in filterOptions" 
                :key="f.type" 
                :label="f.display" 
                @click.stop="f.active = !f.active" 
                :class="{active: f.active}"  />
            </div>
            <InfoDialog ref="infoDialog"/>
        </div>
    </Suspense>
</template>

<script lang="ts">

// Foreign stuff
import { defineComponent, Ref, ref, computed } from "vue";
import moment from "moment";

// Our stuff
import { currentTranslation } from "@/translations";
import { api, Appointment, AppointmentStatus} from "@/api"

// Foreign components
import Button from 'primevue/button';
import Chip from 'primevue/chip';

// Our components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "AppointmentList",
  components: {
      InfoDialog,
      Button,
      Chip
  },
  props: {
      bookingURL: String,
  },
  async setup(props) {
    const infoDialog = ref(false);
    const appointments : Ref<Appointment[]> = ref([] as any);

    //appointments.value = await api.getAppointments(props.bookingURL ?? "abcde"); // TODO: Booking ID Logic
    const temp = await api.getAppointments(props.bookingURL ?? "abcde"); // TODO: Booking ID Logic
    (window as any).res = temp
    appointments.value = temp
  

    const filterOptions = ref(Object.entries(AppointmentStatus)
      .map(([k,v]) => { return {
        display: currentTranslation.appointmentState[v],
        type: v,
        active: true
      }; }));

    const filteredAppointments = computed(() => {
      return appointments.value.filter(e => filterOptions.value.find(f => e.status==f.type && f.active) != undefined);
    });

    const toDateString = (startTime: Date): string => {
      return startTime.toLocaleDateString(navigator.language, {day: '2-digit', month: '2-digit', year: '2-digit'});
    };

    const toTimeString = (time: Date): string => {
      return time.toLocaleTimeString(navigator.language, {hour: '2-digit', minute:'2-digit'});
    };

    return {
      appointments,
      filteredAppointments,
      currentTranslation,
      infoDialog,
      moment,
      filterOptions,
      toDateString,
      toTimeString
    }
  }
});
</script>

<style scoped>
.p-card {

  box-shadow: 0 2px 1px -1px rgba(0, 0, 0, 0.2), 0 1px 5px 0 rgba(0, 0, 0, 0.14), 0 1px 5px 0 rgba(0, 0, 0, 0.15);

  margin-bottom: 0.5em;
  display: grid;
  grid-template-columns: 1fr 1fr 96px;

  grid-template-areas: 
  "date time controls" 
  "status status controls";
}

.p-card > *:not(.controls) {
  margin: 0.5em;
}

.p-card .date {
  grid-area: date;
}

.p-card .time {
  grid-area: time;
}

.p-card .status {
  grid-area: status;
}

.p-card .status::before {
  display: block;
  content: '';
  background-color: lightgray;
  width: 100%;
  height: 1px;
  position: relative;
  top: -0.5em;
}

/* === CARD CONTROLS === */
.p-card .controls {
  grid-area: controls;
  justify-self: stretch;
  width: 100%;
  /*height: 100%;*/
}

.p-card .controls .p-button {
  height: 100%;
}

.p-card .controls .approve {
  background-color: var(--green-500);
}

.p-card .controls .reject {
  background-color: var(--pink-500);
}

.p-card .controls .approved,
.p-card .controls .pending,
.p-card .controls .rejected {
  width: 100%;
}

.p-card .controls .approved {
  color: var(--green-500);
  border: solid 3px var(--green-500);
  border-radius: 3px;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.p-card .controls .pending {
  background-color: var(--orange-400);
}

.p-card .controls .p-button-outlined {
  border-width: 3px;
}

.p-chip {
  margin: 0.3em;
  cursor: pointer;
  user-select: none;
}

.p-chip.active {
  background-color: var(--cyan-500);
}


</style>