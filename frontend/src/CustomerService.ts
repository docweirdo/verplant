import { Ref, ref } from "vue";
import { Appointment, api } from "@/api";
import deepEqual from 'fast-deep-equal';

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

  async sendChanges() {

    const added: Appointment[] = [];
    const revoked: Appointment[] = [];
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
        revoked.push(apptmnt)
      } else if (!deepEqual(newApptmnt, apptmnt)) {
        updated.push(newApptmnt)
      }
    } 

    console.log('updated', updated)
    console.log('added', added)
    console.log('revoked', revoked)
  }
}

const service = new CustomerService();

(window as any).CustomerService = service;

export default service