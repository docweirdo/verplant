import { Ref, ref, toRaw, watch } from "vue";
import { Appointment, api } from "@/api";
import deepEqual from "fast-deep-equal";
import store from "./store";
import router from "./router";

class CustomerService {
  private originalAppointments: Appointment[];
  appointments: Ref<Appointment[]> = ref([]);
  listUpdating = ref(false);

  constructor() {
    this.originalAppointments = [];
    watch(store.bookingUrl, (val, oldVal) => {
      this.onBookingUrlChanged(val, oldVal);
      console.log("Booking URL changed:", val)
    });
  }

  async onBookingUrlChanged(
    url: string | null,
    oldUrl?: string | null
  ): Promise<void> {
    if (url && url !== oldUrl) {
      try {
        await this.fetchBookingInfo();
        await this.fetchAppointments();
      } catch (e) {
        router.push('/TODO error handling'); // TODO: error handling
      }
    }
  }

  async fetchBookingInfo(): Promise<void> {
    if (!store.bookingUrl.value) {
      console.error(
        "CustomerService.fetchBookingInfo was called, but store.bookingUrl was null"
      );
      return;
    }
    store.contactInformations = await api.getBookingInfo(store.bookingUrl.value);
  }

  async fetchAppointments(): Promise<void> {
    if (!store.bookingUrl.value) {
      console.error(
        "CustomerService.fetchAppointments was called, but store.bookingUrl was null"
      );
      return;
    }
    this.listUpdating.value = true;
    this.originalAppointments = await api.getAppointments(
      store.bookingUrl.value
    );
    this.appointments.value = this.originalAppointments.map((a) => {  // Deep copy of List of Objects
      return { ...a };
    });
    this.listUpdating.value = false;
  }

  /**
   * @returns URL for the new booking
   */
  async newBooking(): Promise<string> {
    const url = await api.createNewBooking(toRaw(store.contactInformations));
    console.log(url);

    await this.sendChanges(url);
    return url;
  }

  getChangedAppointments(): {
    added: Appointment[];
    withdrawn: Appointment[];
    updated: Appointment[];
  } {
    const added: Appointment[] = [];
    const withdrawn: Appointment[] = [];
    const updated: Appointment[] = [];

    const localApptmtState: Map<number, Appointment> = new Map();

    for (const apptmntProxy of this.appointments.value) {
      const apptmnt = { ...toRaw(apptmntProxy) };
      if (apptmnt.id === undefined) {
        added.push(apptmnt);
      } else {
        localApptmtState.set(apptmnt.id, apptmnt);
      }
    }

    // assumption: all appointments from the api have an id set
    for (const apptmnt of this.originalAppointments) {
      const newApptmnt = localApptmtState.get(apptmnt.id as number);
      if (!newApptmnt) {
        // appointment was in original, but i not in local state
        withdrawn.push(apptmnt);
      } else if (!deepEqual(newApptmnt, apptmnt)) {
        updated.push(newApptmnt);
      }
    }

    return {
      added,
      withdrawn,
      updated,
    };
  }

  async sendChanges(overwriteUrl?: string) {
    const url = overwriteUrl ?? store.bookingUrl.value;

    if (!url) {
      console.error(
        "CustomerService.sendChanges() called, but neither store.bookingUrl nor overwriteUrl were defined"
      );
      return;
    }

    const { added, withdrawn, updated } = this.getChangedAppointments();

    console.log("updated", updated);
    console.log("added", added);
    console.log("withdrawn", withdrawn);
    const apiCalls = [];

    if (updated.length) {
      apiCalls.push(api.updateAppointments(url, updated));
    }
    if (added.length) {
      apiCalls.push(api.addAppointments(url, added));
    }
    if (withdrawn.length) {
      apiCalls.push(api.withdrawAppointments(url, withdrawn));
    }

    if (apiCalls.length) {
      try {
        await Promise.all(apiCalls);
        await this.fetchAppointments();
      } catch (error) {
        console.error(error); // TODO: popup on error?
      }
    }
  }
}

const service = new CustomerService();

(window as any).CustomerService = service;

export default service;
