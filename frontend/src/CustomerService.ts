import { Ref, ref } from "vue";
import { Appointment, api } from "@/api";
import deepEqual from 'fast-deep-equal';
import store from "./store";

class CustomerService {
  private originalAppointments: Appointment[];
  appointments: Ref<Appointment[]> = ref([]);

  constructor() {
    this.originalAppointments = [];
    this.fetchAppointments()
  }

  async fetchAppointments(): Promise<void> {
    this.originalAppointments = await api.getAppointments('abcde')
    this.appointments.value = this.originalAppointments.map(a => {return {...a}})
  }

  async newBooking(){
    // url = await api.newBooking(...)
    const url = "abcde"
    store.bookingUrl.value = url;

    // api.addAppointments(url, ...)
  }

  async sendChanges() {

    const added: Appointment[] = [];
    const withdrawn: Appointment[] = [];
    const updated: Appointment[] = [];

    const newMap = new Map<number, Appointment>()

    const changes: Appointment[] = [];
    for (const e of this.appointments.value) { changes.push(Object.assign({}, e)) }

    //const changes = this.appointments.value[Symbol.for('ORIGINAL') as unknown as number] as unknown as Appointment[];

    console.log(changes);
    console.log(this.originalAppointments);
    
    
    for (const apptmnt of changes) {
      if (!apptmnt.id) {
        added.push(apptmnt);
      } else {
        newMap.set(apptmnt.id, apptmnt)
      }
    }

    // assumption: all appointments from the api have an id set
    for (const apptmnt of this.originalAppointments) {
      const newApptmnt = newMap.get(apptmnt.id as number)
      //debugger
      if (!newApptmnt) {
        withdrawn.push(apptmnt)
      } else if (!deepEqual(newApptmnt, apptmnt)) {
        updated.push(newApptmnt)
      }
    } 

    console.log('updated', updated)
    console.log('added', added)
    console.log('withdrawn', withdrawn)
    const apiCalls = [];

    if (updated.length){
      apiCalls.push(api.updateAppointments(store.bookingUrl.value as string, updated))
    }
    if (added.length){
      apiCalls.push(api.addAppointments(store.bookingUrl.value as string, added))
    }
    if (withdrawn.length){
      apiCalls.push(api.withdrawAppointments(store.bookingUrl.value as string, withdrawn))
    }

    if (apiCalls.length){
      try {
        await Promise.all(apiCalls);
        await this.fetchAppointments();
      } catch (error) {
        console.error(error)
      }
    }
  }
}

const service = new CustomerService();

(window as any).CustomerService = service;

export default service