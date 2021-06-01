import { reactive, ref, Ref } from "@vue/reactivity";
import { BookingData, Course, api } from '@/api'


class VerplantStore {
  contactInformations: BookingData = reactive({
    firstname: "",
    lastname: "",
    email: "",
    phone: "",
    organisation: "",
    group: "",
    acceptedLegalNotice: false,
    groupSize: 0,
    selectedCourse: undefined
  });

  allCourses: Ref<Course[]> = ref([]);

  bookingUrl = ref<null | string>(null);


  constructor() {
    api.getCourses().then(allCourses => {
      this.allCourses.value = allCourses
    })
  }
}
const store = new VerplantStore();
(window as any).store = store
export default store
