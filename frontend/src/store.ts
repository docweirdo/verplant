import { reactive, ref } from "@vue/reactivity";

class VerplantStore {
  contactInformations = reactive({
    firstname: "",
    lastname: "",
    email: "",
    phone: "",
    organisation: "",
    group: "",
    acceptedLegalNotice: false,
    groupSize: 0,
  });
  
  bookingUrl = ref<null | string>(null)
}

export default new VerplantStore();
