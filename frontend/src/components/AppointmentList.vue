<template>
  <Suspense>
    <template #default>
      <div class="appointment-list" v-bind="$attrs">
        <div
          id="appointments-field"
          class="p-inputtextarea p-inputtext p-component"
        >
          <div
            class="p-component p-card p-shadow-10"
            v-for="appointment in filteredAppointments"
            :key="appointment.id"
          >
            <div class="date">{{ toDateString(appointment.starttime) }}</div>
            <div class="time">
              {{ toTimeString(appointment.starttime) }} -
              {{ toTimeString(appointment.endtime) }}
            </div>
            <div class="status">
              {{
                currentTranslation.appointmentState[appointment.status] ??
                appointment.status
              }}
            </div>
            <div class="controls" v-if="appointment.status != ''">
              <span
                class="p-buttonset"
                v-if="appointment.status == 'SUGGESTED'"
              >
                <Button
                  class="approve"
                  icon="pi pi-check"
                  @click="
                    appointmentChanged(appointment, AppointmentStatus.Approved)
                  "
                />
                <Button
                  class="reject"
                  icon="pi pi-times"
                  @click="
                    appointmentChanged(appointment, AppointmentStatus.Rejected)
                  "
                />
              </span>
              <i
                class="approved pi pi-check-circle"
                v-else-if="appointment.status == 'APPROVED'"
              />
              <Button
                class="pending"
                v-else-if="appointment.status == 'PENDING'"
                icon="pi pi-undo"
                @click="appointmentChanged(appointment, 'WITHDRAWN')"
              />
              <Button
                class="rejected"
                disabled
                v-else-if="appointment.status == 'REJECTED'"
                icon="pi pi-times"
              />
            </div>
          </div>
          <div class="empty" v-if="appointments.length == 0">
            <i class="pi pi-info-circle"></i>
            {{ currentTranslation.noAppointmentsMessage }}
          </div>
        </div>
        <div class="between-slot">
          <slot name="between"></slot>
        </div>
        <label for="filters" v-if="showFilters && visibleSelectors.length > 1"
          >Filter</label
        >
        <div id="filters" v-if="showFilters && visibleSelectors.length > 1">
          <Button
            v-for="f in visibleSelectors"
            :key="f.type"
            :label="f.display"
            @click.stop="f.active = !f.active"
            :class="{ active: f.active }"
          />
        </div>
        <InfoDialog ref="infoDialog" />
      </div>
    </template>
    <template #fallback>
      <div class="fallback">
        <progress-spinner />
        <p>Loading...</p>
      </div>
    </template>
  </Suspense>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, Ref, ref, computed } from "vue";

// Our stuff
import { currentTranslation } from "@/translations";
import { Appointment, AppointmentStatus } from "@/api";
import * as utils from "@/utils";

// Foreign components
import Button from "primevue/button";
import ProgressSpinner from "primevue/progressspinner";

// Our components
import InfoDialog from "@/components/InfoDialog.vue";

export default defineComponent({
  name: "AppointmentList",
  components: {
    InfoDialog,
    Button,
    ProgressSpinner,
  },
  props: {
    bookingURL: String,
    showFilters: {
      type: Boolean,
      default: true,
    },
    appointments: {
      type: Array as () => Appointment[],
      required: true,
    },
  },
  async setup(props, { emit }) {
    const infoDialog = ref(false);

    // Determine all Selectors by available Stati
    const selectors = ref(
      Object.entries(AppointmentStatus)
        .map(([_k, v]) => {
          return {
            display: currentTranslation.appointmentState[v],
            type: v,
            active: true,
          };
        })
    );

    // Filter for each selector if at least one appointment has the status
    const visibleSelectors = computed(() => {
      return selectors.value.filter((selector) => {
          return props.appointments.some(
            (apptmnt) => apptmnt.status === selector.type
          );
        })
    });

    // Apply filters
    const filteredAppointments = computed(() => {
      return props.appointments.filter(
        (e) =>
          visibleSelectors.value.find((f) => e.status == f.type && f.active) !=
          undefined
      );
    });

    const appointmentChanged = (
      appointment: Appointment,
      action: AppointmentStatus | "WITHDRAWN"
    ) => {
      let appointments: Appointment[];
      if (action === "WITHDRAWN") {
        appointments = props.appointments.filter((e) => e !== appointment);
      } else {
        appointments = props.appointments.map((e) => {
          if (e == appointment) {
            switch (action) {
              case AppointmentStatus.Pending:
              case AppointmentStatus.Suggested:
                console.warn(
                  `appointmentChanged was called with invalid action ${action} for appointment ${e.id}`
                );
                break;
              default:
                e.status = action;
                break;
            }
            return e;
          } else {
            // for appointments not matching the event
            return e;
          }
        });
      }

      emit("update:appointments", appointments);
    };

    return {
      AppointmentStatus,
      appointmentChanged,
      filteredAppointments,
      currentTranslation,
      infoDialog,
      visibleSelectors,
      toDateString: utils.toDateString,
      toTimeString: utils.toTimeString,
    };
  },
});
</script>

<style scoped>
.appointment-list {
  display: grid;
  min-height: 0;
  height: 100%;
  grid-template-rows: auto min-content min-content min-content;
}

#appointments-field {
  height: 100%;
  overflow-y: scroll;
}

@media only screen and (max-height: 700px) {
  #appointments-field {
    height: unset;
    max-height: unset;
    overflow-y: unset;
  }
}


.p-card {
  box-shadow: 0 2px 1px -1px rgba(0, 0, 0, 0.2), 0 1px 5px 0 rgba(0, 0, 0, 0.14),
    0 1px 5px 0 rgba(0, 0, 0, 0.15);

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
  content: "";
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

.p-card .controls > .p-buttonset {
  display: grid;
  grid-template-columns: 1fr 1fr;
  width: 100%;
  height: 100%;
}

.p-card .controls .p-button {
  height: 100%;
  width: auto;
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

.p-card .between-slot {
  justify-self: end;
  margin: 0.5em 1em;
}

label[for="filters"] {
  margin: 0.2em 0;
}

#filters {
  display: flex;
  gap: 0.3em;
  margin-bottom: 0.5em;
  flex-wrap: wrap;
}

#filters .p-button {
  background-color: gray;
}

#filters .p-button.active {
  background-color: var(--accentColor);
  color: white;
}


/* === MOBILE === */

@media only screen and (max-width: 520px) {
  .p-card {

  grid-template-columns: 1fr 1fr 1fr;
  

  grid-template-areas:
    "date status status"
    "time controls controls";
  }

  .p-card .status::before {
    content: none;
  }
}
</style>
